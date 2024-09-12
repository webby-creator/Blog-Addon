import { vitePreprocess } from '@sveltejs/vite-plugin-svelte';

export default {
    compilerOptions: {
        css: 'injected',
        dev: true
    },
    // Consult https://svelte.dev/docs#compile-time-svelte-preprocess
    // for more information about preprocessors
    preprocess: vitePreprocess(),
}
