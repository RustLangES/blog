
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
      backgroundImage: (theme) => ({
        "anuncios": "url('https://cdn.discordapp.com/attachments/1148704136704049213/1162664911906811914/DreamShaper_v7_Happy_cute_crabs_in_a_beach_sandy_beach_group_o_1.jpg?ex=653cc314&is=652a4e14&hm=19a710e5133b4a8a7664fc81bf6ce2528137e7c6e5f0f9005f6f11c88ca20856&')",
      }),
      gridTemplateColumns:  (theme) => ({
        "divided": "2.5fr 1fr",
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