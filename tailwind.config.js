
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
      fill: (theme) => ({
        "shape-fill-light": "rgb(203 213 225 / 1)",
        "shape-fill-dark": "rgb(39 39 42 / 1)",
      }),
    },
  },
  plugins: [
   
  ],
};