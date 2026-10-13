import { cn } from '@/lib/utils'

type ProgressBarProps = {
	value: number
	className?: string
	barClassName?: string
}

export function ProgressBar({ value, className, barClassName }: ProgressBarProps) {
	return (
		<div
			role="progressbar"
			aria-valuemin={0}
			aria-valuemax={100}
			aria-valuenow={Math.floor(value * 100)}
			className={cn('bg-muted h-1.5 w-full overflow-hidden rounded-full', className)}
		>
			<div className={cn('bg-primary h-full transition-all', barClassName)} style={{ width: `${value * 100}%` }} />
		</div>
	)
}
