/** @type {import('tailwindcss').Config} */
export default {
    content: ['./src/**/*.{html,js,svelte,ts}'],
    theme: {
        extend: {
            backgroundImage: {
                pattern: "url('/bg.svg')"
            },
            colors: {
                'sky-translucent': 'rgba(14, 165, 233, 0.92)',
                'emerald-translucent': 'rgba(16, 185, 129, 0.92)'
            }
        }
    },
    plugins: [],
    darkMode: 'class'
};
