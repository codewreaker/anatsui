/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./index.html",
    "./src/**/*.{js,ts,jsx,tsx}",
  ],
  theme: {
    extend: {
      colors: {
        'figma-bg': '#2c2c2c',
        'figma-panel': '#1e1e1e',
        'figma-border': '#444444',
        'figma-text': '#ffffff',
        'figma-text-secondary': '#999999',
        'figma-accent': '#0d99ff',
        'figma-selection': '#0d99ff',
      },
      fontFamily: {
        'sans': ['Inter', 'system-ui', 'sans-serif'],
      },
    },
  },
  plugins: [],
};
