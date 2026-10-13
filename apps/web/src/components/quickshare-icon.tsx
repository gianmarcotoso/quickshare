import icon from '@/assets/quickshare-icon.svg'
import { cn } from '@/lib/utils'

export function QuickShareIcon({ className }: { className?: string }) {
	return <img src={icon} alt="" className={cn('shrink-0', className)} />
}
