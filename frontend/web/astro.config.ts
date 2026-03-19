import svelte from '@astrojs/svelte';
import tailwindcss from '@tailwindcss/vite';
import { defineConfig } from 'astro/config';

// https://astro.build/config
export default defineConfig({
	devToolbar: {
		enabled: false,
	},
	integrations: [svelte()],
	vite: {
		define: {
			__APP_VERSION__: JSON.stringify(process.env.npm_package_version ?? ''),
		},
		plugins: [tailwindcss()],
	},
});
