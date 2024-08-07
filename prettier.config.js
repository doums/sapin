/** @type {import("prettier").Config} */

const config = {
  tabWidth: 2,
  semi: true,
  singleQuote: true,
  // this is the default be for the sake of clarity, be explicit
  endOfLine: 'lf',
  // handle svelte files
  plugins: ['prettier-plugin-svelte'],
  overrides: [{ files: '*.svelte', options: { parser: 'svelte' } }],
};

export default config;
