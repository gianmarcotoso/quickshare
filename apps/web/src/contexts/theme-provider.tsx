import { applyThemeToDocument } from '@/lib/apply-theme-to-document'
import { getSystemTheme, subscribeToSystemTheme } from '@/lib/system-theme'
import { useEffect, useState, useSyncExternalStore } from 'react'
import { ThemeContext, type Theme } from './theme-context'

const THEME_STORAGE_KEY = 'theme'

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
