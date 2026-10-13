import { ProgressBar } from '@/components/progress-bar'
import { formatBytes, formatPercent } from '@/lib/format'
import { File } from 'lucide-react'
import { useTranslation } from 'react-i18next'
import type { TrackedFile } from '../utils/tracked-file'

type FileCardProps = {
	item: TrackedFile
	actions?: React.ReactNode
	children?: React.ReactNode
}

export function FileCard({ item, actions, children }: FileCardProps) {
	const { t } = useTranslation()

	return (
		<li className="bg-card flex flex-col gap-3 rounded-xl border p-4 shadow-sm">
			<div className="flex items-start gap-3">
				<File className="text-muted-foreground mt-0.5 size-5 shrink-0" />
				<div className="flex min-w-0 flex-1 flex-col">
					<p className="truncate font-medium" title={item.file.name}>
						{item.file.name}
					</p>
					<p className="text-muted-foreground text-sm">{formatBytes(item.file.size)}</p>
				</div>
				{actions}
			</div>

			<div className="flex flex-col gap-1">
				<p className="text-muted-foreground text-xs font-medium uppercase">SHA-256</p>
				{item.hash ? (
					<p className="font-mono text-xs break-all select-all">{item.hash}</p>
				) : item.hashFailed ? (
					<p className="text-destructive text-xs">{t('uploads.hashError')}</p>
				) : (
					<div className="flex items-center gap-2">
						<ProgressBar value={item.hashProgress} className="h-1" barClassName="bg-muted-foreground" />
						<span className="text-muted-foreground shrink-0 text-xs">
							{t('uploads.hashing', { progress: formatPercent(item.hashProgress) })}
						</span>
					</div>
				)}
			</div>

			{children}
		</li>
	)
}
