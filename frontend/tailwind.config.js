/** @type {import('tailwindcss').Config} */
export const content = [
  "./index.html",
  "./src/**/*.rs",
];
export const theme = {
  extend: {
    colors: {
      warm: {
        50: '#fafaf9',
        100: '#f5f5f4',
        200: '#e7e5e4',
        300: '#d6d3d1',
        400: '#a8a29e',
        500: '#78716c',
        600: '#57534e',
        700: '#44403c',
        800: '#292524',
        900: '#1c1917',
      },
      sage: {
        500: '#84a59d',
        600: '#6b8e85',
      }
    },
    fontFamily: {
      serif: ['Playfair Display', 'serif'],
      sans: ['Inter', 'sans-serif'],
    }
  },
};
export const plugins = [];
