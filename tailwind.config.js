/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./src/**/*.{html,rs}", "./index.html"],
  theme: {
    extend: {},
    theme: {
      extend: {
        textColor: { // [!code focus:5]
          primary: '#1D2129',
          regular: '#4E5969',
          secondary: '#86909C',
          disabled: '#C9CDD4',
        }
      },
    },
  },
  plugins: [],
}