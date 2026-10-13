import { queryClient } from '@/core/http'
import { store } from '@/store/store'
import { QueryClientProvider } from '@tanstack/react-query'
import { Toaster } from 'react-hot-toast'
import { Provider } from 'react-redux'
import { ThemeProvider } from '@/contexts/theme-provider'
import { UploadsPage } from '@/app/features/uploads/uploads.page'

export function Root() {
	return (
		<QueryClientProvider client={queryClient}>
			<ThemeProvider>
				<Provider store={store}>
					<UploadsPage />
					<Toaster />
				</Provider>
			</ThemeProvider>
		</QueryClientProvider>
	)
}
