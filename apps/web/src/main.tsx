import { StrictMode } from 'react'
import { createRoot } from 'react-dom/client'
import './index.css'
import { Root } from './app/root'
import { I18nextProvider } from 'react-i18next'
import { i18n } from './core/i18n'
import { store } from './store/store'
import { LanguageActions } from './store/language.state'

await store.dispatch(LanguageActions.loadLanguage())

createRoot(document.getElementById('root')!).render(
	<StrictMode>
		<I18nextProvider i18n={i18n}>
			<Root />
		</I18nextProvider>
	</StrictMode>,
)
