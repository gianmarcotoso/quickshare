import { AppConfig } from '@/core/app-config'
import { cn } from '@/lib/utils'

export function AppVersion({ className }: { className?: string }) {
	return <p className={cn('text-muted-foreground text-xs', className)}>quickshare {AppConfig.Version}</p>
}
