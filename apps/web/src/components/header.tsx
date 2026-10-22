import { LanguageSwitch } from './language-switch'
import { QuickShareIcon } from './quickshare-icon'
import { ThemeSwitch } from './theme-switch'

export function Header() {
	return (
		<header className="flex items-center justify-between gap-4 py-4">
			<div className="flex items-center gap-3">
				<QuickShareIcon className="size-9" />
				<h1 className="font-display text-2xl font-semibold tracking-tight">quickshare</h1>
			</div>
			<div className="flex items-center">
				<LanguageSwitch />
				<ThemeSwitch />
			</div>
		</header>
	)
}
