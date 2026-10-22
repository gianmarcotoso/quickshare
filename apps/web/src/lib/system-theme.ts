import type { ResolvedTheme } from '@/contexts/theme-context'

const darkSchemeQuery = window.matchMedia('(prefers-color-scheme: dark)')

export function subscribeToSystemTheme(onChange: () => void) {
	darkSchemeQuery.addEventListener('change', onChange)

	return () => darkSchemeQuery.removeEventListener('change', onChange)
}

export function getSystemTheme(): ResolvedTheme {
	return darkSchemeQuery.matches ? 'dark' : 'light'
}
