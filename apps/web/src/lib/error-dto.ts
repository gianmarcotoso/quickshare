import type { ErrorDto } from '@/api/models'
import axios from 'axios'

export function errorDto(error: unknown) {
	return axios.isAxiosError(error) ? (error.response?.data as Partial<ErrorDto> | undefined) : undefined
}
