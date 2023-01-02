import "../styles/globals.css";
import type { AppProps } from "next/app";
import { ControlInterface } from "../components/ControlInterface";
import { ThemeProvider } from "next-themes";

function MyApp({ Component, pageProps }: AppProps) {
	return (
		<ThemeProvider attribute="class">
			<ControlInterface>
				<Component {...pageProps} />
			</ControlInterface>
		</ThemeProvider>
	);
}

export default MyApp;
