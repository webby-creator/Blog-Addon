import type { Config } from 'tailwindcss';
import typography from '@tailwindcss/typography';
import { skeleton } from '@skeletonlabs/tw-plugin';

export default {
    darkMode: 'class',
    content: [
        './src/**/*.{html,js,svelte,ts}'
    ],
    theme: {
        extend: {},
    },
    plugins: [
        typography,
        skeleton({
            themes: {
                preset: [
                    {
                        name: 'skeleton',
                        enhancements: true,
                    }
                ],
            },
        }),
    ],
} satisfies Config;
