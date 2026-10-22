import type { ResolvedTheme } from '@/contexts/theme-context'

const THEME_COLORS: Record<ResolvedTheme, string> = { light: '#f5f7fa', dark: '#14161a' }

export function applyThemeToDocument(theme: ResolvedTheme) {
	document.documentElement.classList.toggle('dark', theme === 'dark')
	document.documentElement.setAttribute('data-theme', theme)
	document
		.querySelectorAll("meta[name='theme-color']")
		.forEach((meta) => meta.setAttribute('content', THEME_COLORS[theme]))
}
