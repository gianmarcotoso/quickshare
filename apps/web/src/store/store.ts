import { configureStore } from '@reduxjs/toolkit'
import { useDispatch, useSelector, type TypedUseSelectorHook } from 'react-redux'
import { languageState } from './language.state'

export const store = configureStore({
	reducer: {
		language: languageState.reducer,
	},
})

export type RootState = ReturnType<typeof store.getState>
export type AppDispatch = typeof store.dispatch

export const useAppSelector = useSelector as TypedUseSelectorHook<RootState>
export const useAppDispatch = useDispatch<AppDispatch>
