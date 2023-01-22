/** @type {import('tailwindcss').Config} */

const plugin = require('tailwindcss/plugin')

module.exports = {
  content: ["./index.html", "./src/**/*.{html,js,rs}"],
  darkMode: "class",
  theme: {
    extend: {
      colors: {
        // Primary tones
        primary: {
          light: "rgb(var(--md-color-primary-light)/ <alpha-value>)",
          dark: "rgb(var(--md-color-primary-dark) / <alpha-value>)",
          on: {
            light: "rgb(var(--md-color-on-primary-light)/ <alpha-value>) ",
            dark: "rgb(var(--md-color-on-primary-dark) / <alpha-value>) ",
          },
          container: {
            light:
              "rgb(var(--md-color-primary-container-light)/ <alpha-value>) ",
            dark: "rgb(var(--md-color-primary-container-dark) / <alpha-value>) ",
            on: {
              light:
                "rgb(var(--md-color-on-primary-container-light)/ <alpha-value>) ",
              dark: "rgb(var(--md-color-on-primary-container-dark) / <alpha-value>) ",
            },
          },
          inverse:
          {
            light:
              "rgb(var(--md-color-inverse-primary-light)/ <alpha-value>) ",
            dark: "rgb(var(--md-color-on-primary-container-dark) / <alpha-value>) ",
          }
        },

        // Secondary tones
        secondary: {
          light: "rgb(var(--md-color-secondary-light)/ <alpha-value>) ",
          dark: "rgb(var(--md-color-secondary-dark) / <alpha-value>) ",
          on: {
            light: "rgb(var(--md-color-on-secondary-light)/ <alpha-value>) ",
            dark: "rgb(var(--md-color-on-secondary-dark) / <alpha-value>) ",
          },
          container: {
            light:
              "rgb(var(--md-color-secondary-container-light)/ <alpha-value>) ",
            dark: "rgb(var(--md-color-secondary-container-dark) / <alpha-value>) ",
            on: {
              light:
                "rgb(var(--md-color-on-secondary-container-light)/ <alpha-value>) ",
              dark: "rgb(var(--md-color-on-secondary-container-dark) / <alpha-value>) ",
            },
          },
        },

        // Tertiary tones
        tertiary: {
          light: "rgb(var(--md-color-tertiary-light)/ <alpha-value>) ",
          dark: "rgb(var(--md-color-tertiary-dark) / <alpha-value>) ",
          on: {
            light: "rgb(var(--md-color-on-tertiary-light)/ <alpha-value>) ",
            dark: "rgb(var(--md-color-on-tertiary-dark) / <alpha-value>) ",
          },
          container: {
            light:
              "rgb(var(--md-color-tertiary-container-light)/ <alpha-value>) ",
            dark: "rgb(var(--md-color-tertiary-container-dark) / <alpha-value>) ",
            on: {
              light:
                "rgb(var(--md-color-on-tertiary-container-light)/ <alpha-value>) ",
              dark: "rgb(var(--md-color-on-tertiary-container-dark) / <alpha-value>) ",
            },
          },
        },

        // Neutral tones (md3 names them as 'surface')
        surface: {
          light: "rgb(var(--md-color-surface-light)/ <alpha-value>) ",
          dark: "rgb(var(--md-color-surface-dark) / <alpha-value>) ",
          on: {
            inverse: {
              light: "rgb(var(--md-color-inverse-on-surface-light)/ <alpha-value>) ",
              dark: "rgb(var(--md-color-inverse-on-surface-dark) / <alpha-value>) ",
            },
            light: "rgb(var(--md-color-on-surface-light)/ <alpha-value>) ",
            dark: "rgb(var(--md-color-on-surface-dark) / <alpha-value>) ",
          },
          // Neutral variant tones
          variant:
          {
            light: "rgb(var(--md-color-neutral-light)/ <alpha-value>) ",
            dark: "rgb(var(--md-color-neutral-dark) / <alpha-value>) ",
            on: {
              light: "rgb(var(--md-color-on-neutral-light)/ <alpha-value>) ",
              dark: "rgb(var(--md-color-on-neutral-dark) / <alpha-value>) ",
            },
          }
        },

         // Background tones
         background: {
          light:
            "rgb(var(--md-color-background-light)/ <alpha-value>) ",
          dark: "rgb(var(--md-color-background-dark) / <alpha-value>) ",
          on: {
            light:
              "rgb(var(--md-color-on-background-light)/ <alpha-value>) ",
            dark: "rgb(var(--md-color-on-background-dark) / <alpha-value>) ",
          },
        },

        // On Error tones
        error:{
          light:
          "rgb(var(--md-color-error-light)/ <alpha-value>) ",
        dark: "rgb(var(--md-color-error-dark) / <alpha-value>) ",
        on: {
          light:
            "rgb(var(--md-color-on-error-light)/ <alpha-value>) ",
          dark: "rgb(var(--md-color-on-error-dark) / <alpha-value>) ",
        },
        container: {
          light:
            "rgb(var(--md-color-error-container-light)/ <alpha-value>) ",
          dark: "rgb(var(--md-color-error-container-light)/ <alpha-value>) ",
          on: {
            light:
              "rgb(var(--md-color-on-error-container-light)/ <alpha-value>) ",
            dark: "rgb(var(--md-color-on-error-container-dark) / <alpha-value>) ",
          },
        },
        },
      },
      fontFamily: {
        sans: ["Graphik", "sans-serif"],
        serif: ["Merriweather", "serif"],
      },
      borderWidth: {
        1: "1px",
      },
    },
  },
  safelist: ["*"],
  plugins: [
    
  ],
};
