import { defineConfig } from 'orval'
import { loadEnv } from 'vite'

const env = loadEnv(process.env.NODE_ENV ?? 'development', import.meta.dirname, '')
const apiUrl = (env.API_URL || 'http://localhost:3030').replace(/\/$/, '')
const spec = env.OPENAPI_SPEC || `${apiUrl}/api/openapi.json`

export default defineConfig({
	quickshare: {
		input: {
			target: spec,
		},
		output: {
			mode: 'tags-split',
			clean: true,
			target: './src/api/client',
			schemas: './src/api/models',
			client: 'react-query',
			httpClient: 'axios',
			baseUrl: '',
			mock: false,
			override: {
				mutator: {
					path: './src/core/http.ts',
					name: 'httpInstance',
				},
				useTypeOverInterfaces: true,
			},
		},
	},
})
