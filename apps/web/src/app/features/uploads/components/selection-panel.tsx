import { ActionButton } from '@/components/action-button'
import { Button } from '@/components/ui/button'
import { formatBytes } from '@/lib/format'
import { Upload, X } from 'lucide-react'
import { useTranslation } from 'react-i18next'
import type { TrackedFile } from '../utils/tracked-file'
import { FileCard } from './file-card'

type SelectionPanelProps = {
	selection: TrackedFile[]
	isUploading: boolean
	onRemove: (id: number) => void
	onClear: () => void
	onUpload: () => void
}

export function SelectionPanel({ selection, isUploading, onRemove, onClear, onUpload }: SelectionPanelProps) {
	const { t } = useTranslation()
	const totalBytes = selection.reduce((total, item) => total + item.file.size, 0)

	return (
		<section className="flex flex-col gap-3">
			<div className="flex items-baseline justify-between gap-2">
				<h2 className="text-lg font-semibold">{t('uploads.selected')}</h2>
				<p className="text-muted-foreground text-sm">
					{t('uploads.selectionSize', { count: selection.length, size: formatBytes(totalBytes) })}
				</p>
			</div>
			<ul className="flex flex-col gap-3">
				{selection.map((item) => (
					<FileCard
						key={item.id}
						item={item}
						actions={
							<ActionButton label={t('uploads.remove')} icon={X} variant="ghost" onClick={() => onRemove(item.id)} />
						}
					/>
				))}
			</ul>
			<div className="flex flex-col-reverse gap-2 sm:flex-row sm:justify-end">
				<Button variant="ghost" onClick={onClear}>
					{t('uploads.clear')}
				</Button>
				<Button size="lg" onClick={onUpload} disabled={isUploading}>
					<Upload /> {t('uploads.upload', { count: selection.length })}
				</Button>
			</div>
			{isUploading && <p className="text-muted-foreground text-right text-sm">{t('uploads.waitForCurrent')}</p>}
		</section>
	)
}
