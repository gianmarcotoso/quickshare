import { ActionButton } from '@/components/action-button'
import { ProgressBar } from '@/components/progress-bar'
import { Alert, AlertDescription, AlertTitle } from '@/components/ui/alert'
import { cn } from '@/lib/utils'
import { RotateCw, TriangleAlert } from 'lucide-react'
import { useTranslation } from 'react-i18next'
import type { BatchInfo, TrackedFile } from '../utils/tracked-file'
import { FileCard } from './file-card'
import { UploadStatusBadge } from './upload-status-badge'

type BatchPanelProps = {
	batch: BatchInfo
	files: TrackedFile[]
	onRetry: (item: TrackedFile) => void
}

export function BatchPanel({ batch, files, onRetry }: BatchPanelProps) {
	const { t, i18n } = useTranslation()
	const uploaded = files.filter((item) => item.status === 'done').length
	const time = batch.startedAt.toLocaleTimeString(i18n.language)

	return (
		<section className="flex flex-col gap-3">
			<div className="flex items-baseline justify-between gap-2">
				<h2 className="text-lg font-semibold">{t('uploads.batch', { time })}</h2>
				<p className="text-muted-foreground text-sm">{t('uploads.progress', { uploaded, total: files.length })}</p>
			</div>

			{batch.isUploading && (
				<Alert variant="warning">
					<TriangleAlert />
					<AlertTitle>{t('uploads.keepOpenTitle')}</AlertTitle>
					<AlertDescription>{t('uploads.keepOpenDescription')}</AlertDescription>
				</Alert>
			)}

			<ul className="flex flex-col gap-3">
				{files.map((item) => (
					<FileCard
						key={item.id}
						item={item}
						actions={
							<>
								<UploadStatusBadge item={item} />
								{item.status === 'error' && !item.retried && (
									<ActionButton
										label={t('uploads.retry')}
										icon={RotateCw}
										variant="outline"
										onClick={() => onRetry(item)}
									/>
								)}
							</>
						}
					>
						{item.status !== 'done' && (
							<ProgressBar value={item.uploadProgress} barClassName={cn(item.status === 'error' && 'bg-destructive')} />
						)}
						{item.error && <p className="text-destructive text-sm">{item.error}</p>}
					</FileCard>
				))}
			</ul>
		</section>
	)
}
