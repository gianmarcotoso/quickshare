import { execFileSync } from 'node:child_process'
import { mkdtempSync, rmSync, writeFileSync } from 'node:fs'
import { tmpdir } from 'node:os'
import path from 'node:path'
import { parseArgs } from 'node:util'

const { values } = parseArgs({
	options: {
		target: { type: 'string' },
		xwin: { type: 'boolean', default: false },
	},
})

const root = path.resolve(import.meta.dirname, '..')
const apiDir = path.join(root, 'apps/api')
const webDir = path.join(root, 'apps/web')
const hostBinary = path.join(apiDir, 'target/release', process.platform === 'win32' ? 'quickshare.exe' : 'quickshare')
const targetArgs = values.target ? ['--target', values.target] : []

function run(command, args, options = {}) {
	execFileSync(command, args, { stdio: 'inherit', ...options })
}

function generateClient() {
	const work = mkdtempSync(path.join(tmpdir(), 'quickshare-build-'))
	const spec = path.join(work, 'openapi.json')

	try {
		writeFileSync(spec, execFileSync(hostBinary, ['openapi'], { maxBuffer: 64 * 1024 * 1024 }))
		run('npm', ['run', 'api-gen'], { cwd: webDir, env: { ...process.env, OPENAPI_SPEC: spec } })
	} finally {
		rmSync(work, { recursive: true, force: true })
	}
}

rmSync(path.join(webDir, 'dist'), { recursive: true, force: true })
run('cargo', ['build', '--release'], { cwd: apiDir })
generateClient()
run('npm', ['run', 'build'], { cwd: webDir })
run('cargo', ['clean', '--release', '-p', 'quickshare', ...targetArgs], { cwd: apiDir })
run('cargo', [...(values.xwin ? ['xwin'] : []), 'build', '--release', ...targetArgs], { cwd: apiDir })
