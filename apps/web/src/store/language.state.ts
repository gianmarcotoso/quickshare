import { createAsyncThunk, createSlice } from '@reduxjs/toolkit'
import i18next from 'i18next'
import localforage from 'localforage'

export type Language = 'it' | 'en'

const DEFAULT_LANGUAGE: Language = window.navigator.language.startsWith('it') ? 'it' : 'en'

export const languageState = createSlice({
	name: 'language',
	initialState: DEFAULT_LANGUAGE,
	reducers: {
		set: (_state, action) => action.payload,
		clear: () => DEFAULT_LANGUAGE,
	},
})

const setLanguage = createAsyncThunk('language/setLanguage', async (language: Language, { dispatch }) => {
	await localforage.setItem('language', language)
	dispatch(languageState.actions.set(language))
})

const clearLanguage = createAsyncThunk('language/clearLanguage', async (_, { dispatch }) => {
	await localforage.removeItem('language')

	dispatch(languageState.actions.clear())
})

const loadLanguage = createAsyncThunk('language/loadLanguage', async (_, { dispatch }) => {
	const language = await localforage.getItem<Language>('language')

	if (language) {
		dispatch(languageState.actions.set(language))
		i18next.changeLanguage(language)
	} else {
		dispatch(languageState.actions.set(DEFAULT_LANGUAGE))
		i18next.changeLanguage(DEFAULT_LANGUAGE)
	}
})

export const LanguageActions = {
	setLanguage,
	clearLanguage,
	loadLanguage,
}
