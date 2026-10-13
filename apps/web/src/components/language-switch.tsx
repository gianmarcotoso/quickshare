import { WithTooltip } from '@/components/with-tooltip'
import { Button } from '@/components/ui/button'
import { DropdownMenu, DropdownMenuContent, DropdownMenuItem, DropdownMenuTrigger } from '@/components/ui/dropdown-menu'
import { cn } from '@/lib/utils'
import { useLanguage } from '@/store/hooks/use-language.hook'
import { Check } from 'lucide-react'
import { GB, IT } from 'country-flag-icons/react/3x2'
import { useTranslation } from 'react-i18next'

const LANGUAGES = [
	{ code: 'it', label: 'Italiano', Flag: IT },
	{ code: 'en', label: 'English', Flag: GB },
] as const

export function LanguageSwitch() {
	const { t } = useTranslation()
	const [language, { setLanguage }] = useLanguage()
	const current = LANGUAGES.find((item) => item.code === language) ?? LANGUAGES[0]

	return (
		<DropdownMenu modal={false}>
			<WithTooltip label={t('language.select')} variant="ghost">
				<DropdownMenuTrigger asChild>
					<Button
						variant="ghost"
						size="icon"
						className="relative scale-95 rounded-full"
						aria-label={t('language.select')}
					>
						<current.Flag className="size-5 rounded-sm" />
					</Button>
				</DropdownMenuTrigger>
			</WithTooltip>
			<DropdownMenuContent align="end">
				{LANGUAGES.map(({ code, label, Flag }) => (
					<DropdownMenuItem key={code} onClick={() => setLanguage(code)}>
						<Flag className="w-5 rounded-sm" /> {label}
						<Check size={14} className={cn('ms-auto', language !== code && 'hidden')} />
					</DropdownMenuItem>
				))}
			</DropdownMenuContent>
		</DropdownMenu>
	)
}
