import type { ErrorCode } from '@/api/models'
import { i18n } from '@/core/i18n'
import en from '@/resources/locales/en.json'
import it from '@/resources/locales/it.json'

export const errorTranslations = { en: en.errors, it: it.errors } satisfies Record<string, Record<ErrorCode, string>>

export function translateError(code: ErrorCode) {
	return i18n.t(`errors.${code}`)
}
