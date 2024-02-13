const defaultTheme = require('tailwindcss/defaultTheme')

/** @type {import('tailwindcss').Config} */
module.exports = {
  content: {
    files: ["*.html", "./src/**/*.rs"],
  },
  theme: {
    fontFamily: {
      "alfa-slab": ["Alfa Slab One", "sans-serif"],
      "fira-sans": ["Fira Sans", "sans-serif"],
      "work-sans": ["Work Sans", "sans-serif"],
    },
    extend: {
      screens: {
        'xs': '475px',
        ...defaultTheme.screens,
      },
      backgroundImage: (theme) => ({
        "anuncios": "url('https://i.imgur.com/tDlT9sr.jpg')",
      }),
      gridTemplateColumns:  (theme) => ({
        "divided": "2.5fr 1fr",
        "sidebar-article": "5rem 1fr"
      }),
      fill: (theme) => ({
        "shape-fill-light": "rgb(203 213 225 / 1)",
        "shape-fill-dark": "rgb(39 39 42 / 1)",
      }),
    },
  },
  plugins: [
    require('@tailwindcss/typography'),
  ],
};