import { closeBatch, createBatch, uploadFile } from '@/api/client/uploads/uploads'
import { errorMessage } from '@/lib/error-message'
import { useEffect, useRef, useState } from 'react'
import toast from 'react-hot-toast'
import { useTranslation } from 'react-i18next'
import { computeSha256 } from '../utils/compute-sha256'
import { trackFile, type BatchInfo, type TrackedFile } from '../utils/tracked-file'

export function useUploads() {
	const { t } = useTranslation()
	const [files, setFiles] = useState<TrackedFile[]>([])
	const [batches, setBatches] = useState<BatchInfo[]>([])
	const [isStarting, setIsStarting] = useState(false)
	const hashQueue = useRef<TrackedFile[]>([])
	const isHashing = useRef(false)
	const selection = files.filter((file) => file.batchId === null)
	const isUploading = isStarting || batches.some((batch) => batch.isUploading)

	useEffect(() => {
		if (!isUploading) {
			return
		}

		const warn = (event: BeforeUnloadEvent) => event.preventDefault()
		window.addEventListener('beforeunload', warn)

		return () => window.removeEventListener('beforeunload', warn)
	}, [isUploading])

	function update(id: number, changes: Partial<TrackedFile>) {
		setFiles((current) => current.map((file) => (file.id === id ? { ...file, ...changes } : file)))
	}

	async function runHashing() {
		if (isHashing.current) {
			return
		}

		isHashing.current = true

		try {
			for (let item = hashQueue.current.shift(); item; item = hashQueue.current.shift()) {
				const { id, file } = item

				try {
					const hash = await computeSha256(file, (hashProgress) => update(id, { hashProgress }))
					setFiles((current) =>
						current.map((tracked) =>
							tracked.file === file && !tracked.hash ? { ...tracked, hash, hashProgress: 1 } : tracked,
						),
					)
				} catch {
					update(id, { hashFailed: true })
				}
			}
		} finally {
			isHashing.current = false
		}
	}

	function select(selected: File[]) {
		if (selected.length === 0) {
			return
		}

		const tracked = selected.map((file) => trackFile(file))

		setFiles((current) => [...current, ...tracked])
		hashQueue.current.push(...tracked)

		void runHashing()
	}

	function remove(id: number) {
		setFiles((current) => current.filter((file) => file.id !== id))
		hashQueue.current = hashQueue.current.filter((file) => file.id !== id)
	}

	function clearSelection() {
		const ids = new Set(selection.map((file) => file.id))

		setFiles((current) => current.filter((file) => !ids.has(file.id)))
		hashQueue.current = hashQueue.current.filter((file) => !ids.has(file.id))
	}

	function retry(failed: TrackedFile) {
		update(failed.id, { retried: true })
		setFiles((current) => [...current, trackFile(failed.file, failed.hash)])
	}

	async function upload() {
		if (selection.length === 0 || isUploading) {
			return
		}

		const queued = selection
		setIsStarting(true)

		let batchId: string

		try {
			batchId = (await createBatch()).id
		} catch (error) {
			toast.error(errorMessage(error, t('uploads.batchError')))
			return
		} finally {
			setIsStarting(false)
		}

		const ids = new Set(queued.map((file) => file.id))
		setBatches((current) => [{ id: batchId, startedAt: new Date(), isUploading: true }, ...current])
		setFiles((current) => current.map((file) => (ids.has(file.id) ? { ...file, batchId } : file)))

		let uploaded = 0

		try {
			for (const { id, file } of queued) {
				update(id, { status: 'uploading', uploadProgress: 0 })

				try {
					const received = await uploadFile(
						batchId,
						{ file },
						{
							onUploadProgress: (event) => update(id, { uploadProgress: event.total ? event.loaded / event.total : 0 }),
						},
					)
					update(id, { status: 'done', uploadProgress: 1, received })
					uploaded += 1
				} catch (error) {
					update(id, { status: 'error', error: errorMessage(error, t('uploads.uploadError')) })
				}
			}
		} finally {
			await closeBatch(batchId).catch(() => undefined)
			setBatches((current) => current.map((batch) => (batch.id === batchId ? { ...batch, isUploading: false } : batch)))
		}

		if (uploaded === queued.length) {
			toast.success(t('uploads.uploaded', { count: uploaded }))
		} else {
			toast.error(t('uploads.partiallyUploaded', { uploaded, total: queued.length }))
		}
	}

	return { files, batches, selection, isUploading, select, remove, clearSelection, retry, upload }
}
