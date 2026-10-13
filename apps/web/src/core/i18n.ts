import i18next from 'i18next'
import { initReactI18next } from 'react-i18next'
import en from '@/resources/locales/en.json'
import it from '@/resources/locales/it.json'

const DefaultLocalizationConfig = {
	resources: {
		it: { translation: it },
		en: { translation: en },
	},
	lng: window.navigator.language.startsWith('it') ? 'it' : 'en',
	fallbackLng: 'en',
	interpolation: {
		escapeValue: false,
	},
}

i18next.use(initReactI18next).init(DefaultLocalizationConfig)

i18next.on('languageChanged', (language) => {
	document.documentElement.lang = language
})

document.documentElement.lang = DefaultLocalizationConfig.lng

export const i18n = i18next
