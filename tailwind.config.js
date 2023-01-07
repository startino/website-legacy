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
        primary: {
          purple:
          {
            light: "#7e22ce",
            DEFAULT:"#7e22ce",
            dark:"#7e22ce",
          },
          fuchsia: 
          {
            light: "#a21caf",
            DEFAULT:"#a21caf",
            dark:"#a21caf"
          },
          pink: 
          {
            light: "#B3185C",
            DEFAULT:"#3C0C65",
            dark:"#5B0028"
          }
        },
        secondary: {

        },
        accent: {
          teal: {
            light: "#14b8a6",
            DEFAULT: "#14b8a6",
            dark: "#14b8a6",
          },
          emerald:
          {
            light: "#10b981",
            DEFAULT: "#10b981",
            dark: "#10b981",
          }
          
        },
      },
    },
  },
  safelist: ["*"],
  plugins: [],
};
