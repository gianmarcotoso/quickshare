import { errorDto } from './error-dto'
import { translateError } from './translate-error'

export function errorMessage(error: unknown, fallback: string) {
	const data = errorDto(error)

	return data?.code ? translateError(data.code) : fallback
}
