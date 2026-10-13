import { Alert, AlertDescription, AlertTitle } from '@/components/ui/alert'
import { Link2Off } from 'lucide-react'
import { useTranslation } from 'react-i18next'

export function InvalidLink() {
	const { t } = useTranslation()

	return (
		<Alert variant="destructive">
			<Link2Off />
			<AlertTitle>{t('uploads.invalidLinkTitle')}</AlertTitle>
			<AlertDescription>{t('uploads.invalidLinkDescription')}</AlertDescription>
		</Alert>
	)
}
