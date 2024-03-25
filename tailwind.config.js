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
  fontSize: {
      sm: '0.8rem',
      base: '1rem',
      xl: '1.25rem',
      '2xl': '1.563rem',
      '3xl': '1.953rem',
      '4xl': '2.441rem',
      '5xl': '3.052rem',
      'half': '3.5rem',
      '3.5xx': '3.5rem',
    },
    extend: {
      screens: {
        'xs': '475px',
        ...defaultTheme.screens,
      },
       lineHeight: {
        'extra-loose': '2.5',
        '12': '3rem',
        '3lh': '2.8rem',
      },
  
      backgroundImage: (theme) => ({
        "anuncios": "url('https://i.imgur.com/tDlT9sr.jpg')",
        "kaku-dev": "url('https://www.rustlang-es.org/kaku.avif')",
        "kaku": "url('https://www.rustlang-es.org/kaku.avif')",
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
