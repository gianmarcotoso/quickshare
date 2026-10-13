import { useEffect } from 'react'
import { LanguageActions, type Language } from '../language.state'
import { useAppDispatch, useAppSelector } from '../store'
import { useTranslation } from 'react-i18next'

export function useLanguage() {
	const language = useAppSelector((state) => state.language)
	const dispatch = useAppDispatch()
	const { i18n } = useTranslation()

	useEffect(() => {
		void i18n.changeLanguage(language)
	}, [language, i18n])

	function setLanguage(lang: Language) {
		dispatch(LanguageActions.setLanguage(lang))
	}

	function clearLanguage() {
		dispatch(LanguageActions.clearLanguage())
	}

	return [language, { setLanguage, clearLanguage }] as const
}
