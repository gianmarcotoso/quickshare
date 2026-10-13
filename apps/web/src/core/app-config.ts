function readVersion() {
	return document.querySelector<HTMLMetaElement>('meta[name="quickshare-version"]')?.content ?? ''
}

function readToken() {
	return new URLSearchParams(window.location.search).get('t') ?? ''
}

export const AppConfig = {
	ApiPath: '/api',
	Version: readVersion(),
	Token: readToken(),
}
