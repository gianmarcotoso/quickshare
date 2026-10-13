import { QueryClient } from '@tanstack/react-query'
import axios, { type AxiosRequestConfig } from 'axios'
import { AppConfig } from './app-config'

export const queryClient = new QueryClient({
	defaultOptions: {
		queries: {
			retry: false,
		},
	},
})

export const http = axios.create({
	baseURL: AppConfig.ApiPath,
	headers: { Authorization: `Bearer ${AppConfig.Token}` },
})

export const httpInstance = <T>(
	config: AxiosRequestConfig,
	options?: AxiosRequestConfig & { signal?: AbortSignal },
): Promise<T> => {
	return http({
		...config,
		...options,
		signal: options?.signal,
	}).then((response) => response.data)
}
