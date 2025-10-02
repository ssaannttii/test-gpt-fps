module.exports = {
  root: true,
  parser: '@typescript-eslint/parser',
  plugins: ['svelte'],
  extends: ['eslint:recommended', 'plugin:svelte/recommended', 'prettier'],
  ignorePatterns: ['dist'],
  overrides: [
    {
      files: ['*.svelte'],
      parser: 'svelte-eslint-parser'
    }
  ],
  env: {
    browser: true,
    es2021: true
  }
};
