import { Button } from '@/components/ui/button'
import { cn } from '@/lib/utils'
import { Upload } from 'lucide-react'
import { useRef, useState } from 'react'
import { useTranslation } from 'react-i18next'

type FileDropZoneProps = {
	onFiles: (files: File[]) => void
}

export function FileDropZone({ onFiles }: FileDropZoneProps) {
	const { t } = useTranslation()
	const input = useRef<HTMLInputElement>(null)
	const [isOver, setIsOver] = useState(false)

	return (
		<div
			className={cn(
				'bg-card flex flex-col items-center gap-4 rounded-xl border-2 border-dashed px-6 py-10 text-center transition-colors',
				isOver && 'border-primary bg-accent',
			)}
			onDragOver={(event) => {
				if (event.dataTransfer.types.includes('Files')) {
					event.preventDefault()
					setIsOver(true)
				}
			}}
			onDragLeave={() => setIsOver(false)}
			onDrop={(event) => {
				event.preventDefault()
				setIsOver(false)
				onFiles([...event.dataTransfer.files])
			}}
		>
			<div className="bg-primary/10 text-primary flex size-14 items-center justify-center rounded-full">
				<Upload className="size-7" />
			</div>
			<div className="flex flex-col gap-1">
				<p className="font-medium">{t('uploads.dropHere')}</p>
				<p className="text-muted-foreground text-sm">{t('uploads.dropDescription')}</p>
			</div>
			<Button size="lg" onClick={() => input.current?.click()}>
				{t('uploads.choose')}
			</Button>
			<input
				ref={input}
				type="file"
				multiple
				className="hidden"
				onChange={(event) => {
					onFiles([...(event.target.files ?? [])])
					event.target.value = ''
				}}
			/>
		</div>
	)
}
