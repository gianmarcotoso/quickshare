import { useCheckSession } from '@/api/client/uploads/uploads'
import { AppVersion } from '@/components/app-version'
import { Header } from '@/components/header'
import { Loader2 } from 'lucide-react'
import { useTranslation } from 'react-i18next'
import { BatchPanel } from './components/batch-panel'
import { FileDropZone } from './components/file-drop-zone'
import { InvalidLink } from './components/invalid-link'
import { SelectionPanel } from './components/selection-panel'
import { useUploads } from './hooks/use-uploads.hook'

export function UploadsPage() {
	const { t } = useTranslation()
	const session = useCheckSession()
	const { files, batches, selection, isUploading, select, remove, clearSelection, retry, upload } = useUploads()

	return (
		<div className="mx-auto flex min-h-svh w-full max-w-2xl flex-col gap-6 px-4 pb-6">
			<Header />

			{session.isPending ? (
				<Loader2 className="text-muted-foreground mx-auto size-6 animate-spin" />
			) : session.isError ? (
				<InvalidLink />
			) : (
				<>
					<p className="text-muted-foreground">{t('uploads.description')}</p>

					<FileDropZone onFiles={select} />

					{selection.length > 0 && (
						<SelectionPanel
							selection={selection}
							isUploading={isUploading}
							onRemove={remove}
							onClear={clearSelection}
							onUpload={upload}
						/>
					)}

					{batches.map((batch) => (
						<BatchPanel
							key={batch.id}
							batch={batch}
							files={files.filter((file) => file.batchId === batch.id)}
							onRetry={retry}
						/>
					))}
				</>
			)}

			<AppVersion className="mt-auto text-center" />
		</div>
	)
}
