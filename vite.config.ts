import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';

// /** @type {import('vite').Plugin} */
// const viteServerConfig = {
// 	name: 'log-request-middleware',
// 	configureServer(server) {
// 		server.middlewares.use((req, res, next) => {
// 			res.setHeader('Access-Control-Allow-Origin', '*');
// 			res.setHeader('Access-Control-Allow-Methods', 'GET');
// 			res.setHeader('Cross-Origin-Opener-Policy', 'same-origin');
// 			res.setHeader('Cross-Origin-Embedder-Policy', 'require-corp');
// 			next();
// 		});
// 	}
// };

// export default defineConfig({
// 	plugins: [sveltekit()],
// 	define: {
// 		APP_VERSION: JSON.stringify(process.env.npm_package_version),
// 		APP_BUILD_HASH: JSON.stringify(process.env.APP_BUILD_HASH || 'dev-build')
// 	},
// 	build: {
// 		sourcemap: true
// 	},
// 	worker: {
// 		format: 'es'
// 	}
// });

export default defineConfig({
    plugins: [sveltekit()],
	define: {
		APP_VERSION: JSON.stringify(process.env.npm_package_version),
		APP_BUILD_HASH: JSON.stringify(process.env.APP_BUILD_HASH || 'dev-build')
	},
    server: {
        port: 5173,
        strictPort: true,
		fs: {
            allow: ['.'] // Adjust this to allow the paths you need
        },
    },
    build: {
        target: 'esnext',
        minify: !process.env.TAURI_DEBUG ? 'esbuild' : false,
        sourcemap: !!process.env.TAURI_DEBUG,
    },
	worker: {
		format: 'es'
	},
    optimizeDeps: {
        exclude: ['@pyscript/core'],
    },
});
