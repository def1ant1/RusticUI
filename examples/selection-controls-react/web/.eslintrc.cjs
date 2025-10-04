module.exports = {
  root: true,
  parserOptions: {
    project: './tsconfig.json',
    ecmaVersion: 'latest',
    sourceType: 'module'
  },
  env: {
    browser: true,
    es2022: true,
    jest: true
  },
  extends: ['eslint:recommended', 'plugin:react-hooks/recommended', 'prettier'],
  plugins: ['react-refresh'],
  rules: {
    'react-refresh/only-export-components': 'off'
  }
};
