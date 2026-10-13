import { defineConfig, loadEnv, type Plugin } from 'vite'
import tailwindcss from '@tailwindcss/vite'
import react, { reactCompilerPreset } from '@vitejs/plugin-react'
import babel from '@rolldown/plugin-babel'
import path from 'node:path'
import { readFileSync } from 'node:fs'

const API_PATH = '/api'

function injectDevVersion(version: string): Plugin {
	return {
		name: 'quickshare-dev-version',
		apply: 'serve',
		transformIndexHtml: () => [
			{ tag: 'meta', attrs: { name: 'quickshare-version', content: version }, injectTo: 'head-prepend' },
		],
	}
}

function readServerVersion() {
	const manifest = readFileSync(path.resolve(import.meta.dirname, '../api/Cargo.toml'), 'utf8')

	return manifest.match(/^version = "(.+)"$/m)?.[1] ?? ''
}

export default defineConfig(({ command, mode }) => {
	const env = loadEnv(mode, import.meta.dirname, '')
	const appPort = Number(env.APP_LOCAL_PORT) || 5176
	const apiUrl = (env.API_URL || 'http://localhost:3030').replace(/\/$/, '')

	return {
		base: command === 'build' ? './' : '/',
		plugins: [
			react(),
			babel({ presets: [reactCompilerPreset()] }),
			tailwindcss(),
			injectDevVersion(readServerVersion()),
		],
		resolve: {
			alias: {
				'@': path.resolve(import.meta.dirname, './src'),
			},
		},
		server: {
			host: true,
			port: appPort,
			proxy: {
				[API_PATH]: apiUrl,
			},
		},
	}
})
