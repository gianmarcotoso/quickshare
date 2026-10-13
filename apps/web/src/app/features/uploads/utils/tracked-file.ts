import type { ReceivedFileDto } from '@/api/models'

export type UploadStatus = 'waiting' | 'uploading' | 'done' | 'error'

export type TrackedFile = {
	id: number
	file: File
	hash: string | null
	hashProgress: number
	hashFailed: boolean
	batchId: string | null
	status: UploadStatus
	uploadProgress: number
	received: ReceivedFileDto | null
	error: string | null
	retried: boolean
}

export type BatchInfo = {
	id: string
	startedAt: Date
	isUploading: boolean
}

let nextId = 0

export function trackFile(file: File, hash: string | null = null): TrackedFile {
	nextId += 1

	return {
		id: nextId,
		file,
		hash,
		hashProgress: hash ? 1 : 0,
		hashFailed: false,
		batchId: null,
		status: 'waiting',
		uploadProgress: 0,
		received: null,
		error: null,
		retried: false,
	}
}
