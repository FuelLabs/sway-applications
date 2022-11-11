const colors = require('tailwindcss/colors');

delete colors['lightBlue'];
delete colors['warmGray'];
delete colors['trueGray'];
delete colors['coolGray'];
delete colors['blueGray'];

module.exports = {
  content: ['./src/**/*.{js,ts,jsx,tsx}'],
  theme: {
    extend: {},
    screens: {
      sm: '640px',
      md: '960px',
      lg: '1440px',
    },
    colors: {
      ...colors,
      gray: {
        50: '#E8EAED',
        100: '#D4D8DD',
        200: '#AAB1BB',
        300: '#7C8897',
        400: '#58626F',
        500: '#363C44',
        600: '#2B3036',
        700: '#202328',
        800: '#17191C',
        900: '#0B0D0E',
      },
      primary: {
        50: '#eff6f5',
        100: '#d2eff2',
        200: '#9fe5e2',
        300: '#66cbc0',
        400: '#2aac98',
        500: '#1e9071',
        600: '#1b7958',
        700: '#195d46',
        800: '#134034',
        900: '#0d2727',
      },
    },
    fontFamily: {
      sans: ['InterVariable', 'sans-serif'],
      display: ['RalewayVariable', 'sans-serif'],
    },
  },
  variants: {
    extend: {
      backgroundColor: ['disabled'],
      textColor: ['disabled'],
      scale: ['disabled'],
    },
  },
  plugins: [require('@tailwindcss/typography')],
};
