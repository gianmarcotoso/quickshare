const BYTE_UNITS = ['B', 'KB', 'MB', 'GB', 'TB']

export function formatBytes(bytes: number): string {
	const exponent = Math.min(Math.floor(Math.log(Math.max(bytes, 1)) / Math.log(1024)), BYTE_UNITS.length - 1)
	const value = bytes / 1024 ** exponent

	return `${value.toFixed(exponent === 0 ? 0 : 1)} ${BYTE_UNITS[exponent]}`
}

export function formatPercent(fraction: number): string {
	return `${Math.floor(fraction * 100)}%`
}
