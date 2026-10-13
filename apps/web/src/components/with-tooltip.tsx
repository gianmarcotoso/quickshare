import type { Button } from './ui/button'
import { Tooltip, TooltipContent, TooltipTrigger } from './ui/tooltip'

export type TooltipVariant = NonNullable<React.ComponentProps<typeof Button>['variant']>

const COLORS: Record<TooltipVariant, { content: string; arrow: string }> = {
	default: { content: 'bg-primary text-primary-foreground', arrow: 'bg-primary fill-primary' },
	destructive: { content: 'bg-destructive text-destructive-foreground', arrow: 'bg-destructive fill-destructive' },
	outline: { content: 'bg-primary text-primary-foreground', arrow: 'bg-primary fill-primary' },
	secondary: { content: 'bg-secondary text-secondary-foreground', arrow: 'bg-secondary fill-secondary' },
	ghost: { content: 'bg-primary text-primary-foreground', arrow: 'bg-primary fill-primary' },
	link: { content: 'bg-primary text-primary-foreground', arrow: 'bg-primary fill-primary' },
	success: { content: 'bg-success text-success-foreground', arrow: 'bg-success fill-success' },
	warning: { content: 'bg-warning text-warning-foreground', arrow: 'bg-warning fill-warning' },
	info: { content: 'bg-info text-info-foreground', arrow: 'bg-info fill-info' },
	action: { content: 'bg-action text-action-foreground', arrow: 'bg-action fill-action' },
}

type WithTooltipProps = {
	label: string
	variant?: TooltipVariant | null
	children: React.ReactElement
}

export function WithTooltip({ label, variant, children }: WithTooltipProps) {
	const colors = COLORS[variant ?? 'default']

	return (
		<Tooltip>
			<TooltipTrigger asChild>{children}</TooltipTrigger>
			<TooltipContent className={colors.content} arrowClassName={colors.arrow}>
				{label}
			</TooltipContent>
		</Tooltip>
	)
}
