module.exports = {
  purge: ['./pages/**/*.tsx', './components/**/*.tsx'],
  darkMode: false, // or 'media' or 'class'
  theme: {
    extend: {
      gridTemplateColumns: {
        custom: '1fr 768px 1fr',
      },
      gridTemplateRows: {
        custom: '3rem auto 4rem',
      },
    },
  },
  variants: {
    translate: ['responsive', 'hover', 'focus', 'group-hover'],
    transform: ['responsive', 'group-hover'],
  },
  plugins: [require('@tailwindcss/forms'), require('@tailwindcss/typography')],
}
