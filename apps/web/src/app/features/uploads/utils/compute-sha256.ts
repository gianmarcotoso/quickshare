import { createSHA256 } from 'hash-wasm'

const CHUNK_BYTES = 8 * 1024 * 1024

export async function computeSha256(file: File, onProgress: (fraction: number) => void): Promise<string> {
	const hasher = await createSHA256()
	hasher.init()

	for (let offset = 0; offset < file.size; offset += CHUNK_BYTES) {
		const chunk = await file.slice(offset, offset + CHUNK_BYTES).arrayBuffer()
		hasher.update(new Uint8Array(chunk))
		onProgress(Math.min(offset + CHUNK_BYTES, file.size) / file.size)
	}

	return hasher.digest('hex')
}
