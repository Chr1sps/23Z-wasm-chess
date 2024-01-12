import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vitest/config';

import topLevelAwait from 'vite-plugin-top-level-await';
import initWasm from 'vite-plugin-wasm';
export default defineConfig({
	plugins: [sveltekit(), initWasm(), topLevelAwait()],
	test: {
		include: ['src/**/*.{test,spec}.{js,ts}']
	},
	server: {
		fs: {
			allow: ['./wasm/pkg']
		}
	}
});
