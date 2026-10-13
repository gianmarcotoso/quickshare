import { WithTooltip } from '@/components/with-tooltip'
import { useTranslation } from 'react-i18next'
import { Check, Moon, Sun } from 'lucide-react'
import { cn } from '@/lib/utils'
import { useTheme } from '@/contexts/theme-provider'
import { Button } from '@/components/ui/button'
import { DropdownMenu, DropdownMenuContent, DropdownMenuItem, DropdownMenuTrigger } from '@/components/ui/dropdown-menu'

export function ThemeSwitch() {
	const { t } = useTranslation()
	const { theme, setTheme } = useTheme()

	return (
		<DropdownMenu modal={false}>
			<WithTooltip label={t('theme.toggle')} variant="ghost">
				<DropdownMenuTrigger asChild>
					<Button variant="ghost" size="icon" className="relative scale-95 rounded-full" aria-label={t('theme.toggle')}>
						<Sun className="absolute inset-2 size-[1.2rem] scale-100 rotate-0 transition-all dark:scale-0 dark:-rotate-90" />
						<Moon className="absolute inset-2 size-[1.2rem] scale-0 rotate-90 transition-all dark:scale-100 dark:rotate-0" />
					</Button>
				</DropdownMenuTrigger>
			</WithTooltip>
			<DropdownMenuContent align="end">
				<DropdownMenuItem onClick={() => setTheme('light')}>
					{t('theme.light')} <Check size={14} className={cn('ms-auto', theme !== 'light' && 'hidden')} />
				</DropdownMenuItem>
				<DropdownMenuItem onClick={() => setTheme('dark')}>
					{t('theme.dark')}
					<Check size={14} className={cn('ms-auto', theme !== 'dark' && 'hidden')} />
				</DropdownMenuItem>
				<DropdownMenuItem onClick={() => setTheme('system')}>
					{t('theme.system')}
					<Check size={14} className={cn('ms-auto', theme !== 'system' && 'hidden')} />
				</DropdownMenuItem>
			</DropdownMenuContent>
		</DropdownMenu>
	)
}
