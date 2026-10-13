import { cn } from '@/lib/utils'
import { Button } from './ui/button'
import { WithTooltip } from './with-tooltip'

type ActionButtonProps = Omit<React.ComponentProps<typeof Button>, 'children' | 'size' | 'asChild'> & {
	label: string
	icon: React.ElementType
	href?: string
	download?: string
	external?: boolean
}

export function ActionButton({
	label,
	icon: Icon,
	variant = 'default',
	className,
	href,
	download,
	external,
	...props
}: ActionButtonProps) {
	return (
		<WithTooltip label={label} variant={variant}>
			<Button
				size="icon"
				variant={variant}
				aria-label={label}
				asChild={!!href}
				{...props}
				className={cn('size-8', className)}
			>
				{href ? (
					<a href={href} download={download} {...(external ? { target: '_blank', rel: 'noreferrer' } : {})}>
						<Icon />
					</a>
				) : (
					<Icon />
				)}
			</Button>
		</WithTooltip>
	)
}
