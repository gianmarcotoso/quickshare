import { createContext, useContext, useEffect, useState, useSyncExternalStore } from 'react'

type Theme = 'light' | 'dark' | 'system'
type ResolvedTheme = 'light' | 'dark'

type ThemeContextValue = {
	theme: Theme
	resolvedTheme: ResolvedTheme
	setTheme: (theme: Theme) => void
}

const THEME_STORAGE_KEY = 'theme'
const THEME_COLORS: Record<ResolvedTheme, string> = { light: '#f5f7fa', dark: '#14161a' }
const ThemeContext = createContext<ThemeContextValue | null>(null)
const darkSchemeQuery = window.matchMedia('(prefers-color-scheme: dark)')

function subscribeToSystemTheme(onChange: () => void) {
	darkSchemeQuery.addEventListener('change', onChange)

	return () => darkSchemeQuery.removeEventListener('change', onChange)
}

function getSystemTheme(): ResolvedTheme {
	return darkSchemeQuery.matches ? 'dark' : 'light'
}

function applyThemeToDocument(theme: ResolvedTheme) {
	document.documentElement.classList.toggle('dark', theme === 'dark')
	document.documentElement.setAttribute('data-theme', theme)
	document
		.querySelectorAll("meta[name='theme-color']")
		.forEach((meta) => meta.setAttribute('content', THEME_COLORS[theme]))
}

type ThemeProviderProps = {
	children: React.ReactNode
}

export function ThemeProvider({ children }: ThemeProviderProps) {
	const [theme, setThemeState] = useState<Theme>(() => {
		const saved = localStorage.getItem(THEME_STORAGE_KEY)
		if (saved === 'light' || saved === 'dark' || saved === 'system') {
			return saved
		}

		return 'system'
	})
	const systemTheme = useSyncExternalStore(subscribeToSystemTheme, getSystemTheme)
	const resolvedTheme = theme === 'system' ? systemTheme : theme

	useEffect(() => {
		applyThemeToDocument(resolvedTheme)
	}, [resolvedTheme])

	const setTheme = (nextTheme: Theme) => {
		setThemeState(nextTheme)
		localStorage.setItem(THEME_STORAGE_KEY, nextTheme)
	}

	return <ThemeContext value={{ theme, resolvedTheme, setTheme }}>{children}</ThemeContext>
}

// eslint-disable-next-line react-refresh/only-export-components
export function useTheme() {
	const context = useContext(ThemeContext)

	if (!context) {
		throw new Error('useTheme must be used within a ThemeProvider')
	}

	return context
}
