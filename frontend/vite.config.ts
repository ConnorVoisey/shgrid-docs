import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vitest/config';
export default defineConfig({
	plugins: [sveltekit()],
	server: {
		port: 8000,
	},
	test: {
		include: ['src/**/*.{test,spec}.{js,ts}'],
	},
	css: {
		preprocessorOptions: {
			scss: {
				additionalData: `
             @use 'src/styles/_variables.scss' as *;
        `,
			},
		},
	},
});
