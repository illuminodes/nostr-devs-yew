/** @type {import('tailwindcss').Config} */
module.exports = {
    content: ["./src/**/*.{html,rs}", "./index.html"],
    theme: {
        extend: {
            colors: {
                'nostr-dark': '#4B1862',
                'nostr-light': '#A334D5',
            },
            fontFamily: {
                'source': ['SourceCodePro', 'sans-serif'],
            },
        },
    },
    plugins: [require("@tailwindcss/forms")],
}

