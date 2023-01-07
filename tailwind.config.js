/** @type {import('tailwindcss').Config} */

// tailwind.config.js
const colors = require("tailwindcss/colors");

module.exports = {
  content: ["./index.html", "./src/**/*.{html,js,rs}"],
  darkMode: "class",
  theme: {
    extend: {
      // If we want a more flexible color pallete, you can do " 'pink': colors.pink, ". This allows you to do " text-primary-pink-xxx "
      colors: {
        primary: colors.purple,
        secondary: colors.pink,
        accent: colors.emerald,
      },
    },
  },
  safelist: ["*"],
  plugins: [],
};
