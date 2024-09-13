import adapter from '@sveltejs/adapter-static';
import { vitePreprocess } from '@sveltejs/vite-plugin-svelte';

/** @type {import('@sveltejs/kit').Config} */
const config = {
	extensions: ['.svelte'],
	preprocess: [
		vitePreprocess(),
	],
	kit: {
		adapter: adapter({
			pages: 'build',
			assets: 'build',
			fallback: 'index.html',
			precompress: false,
			strict: true
		}),
		prerender: {
			entries: []
		}
	},
	onwarn: (warning, handler) => {
		const { code, _ } = warning;
		if (code === 'css-unused-selector') return;

		handler(warning);
	}
};

export default config;