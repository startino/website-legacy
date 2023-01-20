/** @type {import('tailwindcss').Config} */

module.exports = {
  content: ["./index.html", "./src/**/*.{html,js,rs}"],
  darkMode: "class",
  theme: {
    extend: {
      colors: {

        // Primary tones
        primary: {
          light: "rgb(var(--md-color-primary-light)",
          dark: "rgb(var(--md-color-primary-dark)",
          on: {
            light: "rgb(var(--md-color-on-primary-light) ",
            dark: "rgb(var(--md-color-on-primary-dark) ",
          },
          container: {
            light:
              "rgb(var(--md-color-primary-container-light) ",
            dark: "rgb(var(--md-color-primary-container-dark) ",
            on: {
              light:
                "rgb(var(--md-color-on-primary-container-light) ",
              dark: "rgb(var(--md-color-on-primary-container-dark) ",
            },
          },
          inverse:
          {
            light:
              "rgb(var(--md-color-inverse-primary-light) ",
            dark: "rgb(var(--md-color-on-primary-container-dark) ",
          }
        },

        // Secondary tones
        secondary: {
          light: "rgb(var(--md-color-secondary-light) ",
          dark: "rgb(var(--md-color-secondary-dark) ",
          on: {
            light: "rgb(var(--md-color-on-secondary-light) ",
            dark: "rgb(var(--md-color-on-secondary-dark) ",
          },
          container: {
            light:
              "rgb(var(--md-color-secondary-container-light) ",
            dark: "rgb(var(--md-color-secondary-container-dark) ",
            on: {
              light:
                "rgb(var(--md-color-on-secondary-container-light) ",
              dark: "rgb(var(--md-color-on-secondary-container-dark) ",
            },
          },
        },

        // Tertiary tones
        tertiary: {
          light: "rgb(var(--md-color-tertiary-light) ",
          dark: "rgb(var(--md-color-tertiary-dark) ",
          on: {
            light: "rgb(var(--md-color-on-tertiary-light) ",
            dark: "rgb(var(--md-color-on-tertiary-dark) ",
          },
          container: {
            light:
              "rgb(var(--md-color-tertiary-container-light) ",
            dark: "rgb(var(--md-color-tertiary-container-dark) ",
            on: {
              light:
                "rgb(var(--md-color-on-tertiary-container-light) ",
              dark: "rgb(var(--md-color-on-tertiary-container-dark) ",
            },
          },
        },

        // Neutral tones (md3 names them as 'surface')
        neutral: {
          light: "rgb(var(--md-color-surface-light) ",
          dark: "rgb(var(--md-color-surface-dark) ",
          on: {
            inverse: {
              light: "rgb(var(--md-color-inverse-on-surface-light) ",
              dark: "rgb(var(--md-color-inverse-on-surface-dark) ",
            },
            light: "rgb(var(--md-color-on-surface-light) ",
            dark: "rgb(var(--md-color-on-surface-dark) ",
          },
          container: {
            light:
              "rgb(var(--md-color-neutral-container-light) ",
            dark: "rgb(var(--md-color-neutral-container-dark) ",
            on: {
              light:
                "rgb(var(--md-color-on-neutral-container-light) ",
              dark: "rgb(var(--md-color-on-neutral-container-dark) ",
            },
          },
          // Neutral variant tones
          variant:
          {
            light: "rgb(var(--md-color-neutral-light) ",
            dark: "rgb(var(--md-color-neutral-dark) ",
            on: {
              light: "rgb(var(--md-color-on-neutral-light) ",
              dark: "rgb(var(--md-color-on-neutral-dark) ",
            },
            container: {
              light:
                "rgb(var(--md-color-surface-variant-light) ",
              dark: "rgb(var(--md-color-surface-variant-dark) ",
              on: {
                light:
                  "rgb(var(--md-color-on-surface-variant-light) ",
                dark: "rgb(var(--md-color-on-surface-variant-dark) ",
              },
            },
          }
        },

         // Background tones
         background: {
          light:
            "rgb(var(--md-color-background-light) ",
          dark: "rgb(var(--md-color-background-dark) ",
          on: {
            light:
              "rgb(var(--md-color-on-background-light) ",
            dark: "rgb(var(--md-color-on-background-dark) ",
          },
        },

        // On Error tones
        error:{
          light:
          "rgb(var(--md-color-error-light) ",
        dark: "rgb(var(--md-color-error-dark) ",
        on: {
          light:
            "rgb(var(--md-color-on-error-light) ",
          dark: "rgb(var(--md-color-on-error-dark) ",
        },
        container: {
          light:
            "rgb(var(--md-color-error-container-light) ",
          dark: "rgb(var(--md-color-error-container-light) ",
          on: {
            light:
              "rgb(var(--md-color-on-error-container-light) ",
            dark: "rgb(var(--md-color-on-error-container-dark) ",
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
  plugins: [],
};
