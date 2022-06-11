import '../styles/globals.css'
import type { AppProps } from 'next/app'
import { ControlInterface } from '../components/ControlInterface'
import { ThemeProvider } from "next-themes";

function MyApp({ Component, pageProps }: AppProps) {
  return (
		<ThemeProvider attribute='class'>
			<div className='h-screen flex dark:bg-dark bg-light text-dark dark:text-light'>
				<div className='z-50'>
					<ControlInterface>
						<Component {...pageProps} />
					</ControlInterface>
				</div>
				<div className='lg:block hidden z-0 h-full border-4 absolute border-dark left-32 dark:border-light'></div>
				<div className='lg:block hidden z-0 h-full border-4 absolute border-dark left-36 dark:border-light'></div>
			</div>
		</ThemeProvider>
  );
}

export default MyApp
