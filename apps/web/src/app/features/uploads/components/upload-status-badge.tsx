import { Badge } from '@/components/ui/badge'
import { formatPercent } from '@/lib/format'
import { CircleAlert, Clock, Loader2, ShieldAlert, ShieldCheck } from 'lucide-react'
import { useTranslation } from 'react-i18next'
import type { TrackedFile } from '../utils/tracked-file'

export function UploadStatusBadge({ item }: { item: TrackedFile }) {
	const { t } = useTranslation()

	switch (item.status) {
		case 'waiting':
			return (
				<Badge variant="outline">
					<Clock /> {t('uploads.status.waiting')}
				</Badge>
			)
		case 'uploading':
			return (
				<Badge variant="info">
					<Loader2 className="animate-spin" /> {formatPercent(item.uploadProgress)}
				</Badge>
			)
		case 'error':
			return (
				<Badge variant="destructive">
					<CircleAlert /> {t('uploads.status.error')}
				</Badge>
			)
		case 'done':
			if (item.hash && item.received && item.hash !== item.received.sha256) {
				return (
					<Badge variant="destructive">
						<ShieldAlert /> {t('uploads.status.mismatch')}
					</Badge>
				)
			}

			return (
				<Badge variant="success">
					<ShieldCheck /> {item.hash ? t('uploads.status.verified') : t('uploads.status.done')}
				</Badge>
			)
	}
}
