module.exports = {
	darkMode: "class",
	content: [
		"./pages/**/*.{js,ts,jsx,tsx}",
		"./components/**/*.{js,ts,jsx,tsx}"
	],
	theme: {
		extend: {
			colors: {
				dark: "#334155",
				light: "#cbd5e1",
				success: "#22c55e",
				error: "#dc2626"
			}
		}
	},
	plugins: []
};
