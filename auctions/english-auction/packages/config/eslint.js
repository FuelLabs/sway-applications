const path = require('path');

const resolveDir = (dir) => path.resolve(__dirname, dir);
const resolveRoot = (dir = '') => resolveDir(path.join('../../', dir));

module.exports = {
  parser: '@typescript-eslint/parser',
  parserOptions: {
    tsconfigRootDir: resolveRoot(),
    project: [resolveRoot('./tsconfig.eslint.json'), resolveRoot('./**/**/tsconfig.json')],
    ecmaVersion: 2020,
    sourceType: 'module',
    ecmaFeatures: {
      jsx: true,
    },
  },
  plugins: ['@typescript-eslint', 'testing-library', 'jest-dom', 'cypress'],
  extends: [
    'airbnb-base',
    'airbnb-typescript/base',
    'plugin:@typescript-eslint/recommended',
    'plugin:eslint-comments/recommended',
    'plugin:react/recommended',
    'prettier',
  ],
  settings: {
    react: {
      version: 'detect',
    },
  },
  rules: {
    '@typescript-eslint/ban-ts-comment': 'off',
    '@typescript-eslint/no-explicit-any': 'error',
    '@typescript-eslint/no-non-null-assertion': 'off',
    '@typescript-eslint/no-use-before-define': 'off',
    '@typescript-eslint/consistent-type-imports': 2,
    '@typescript-eslint/lines-between-class-members': [
      'error',
      'always',
      { exceptAfterSingleLine: true },
    ],
    '@typescript-eslint/no-inferrable-types': 'off',
    '@typescript-eslint/no-var-requires': 'off',
    'class-methods-use-this': 'off',
    'eslint-comments/disable-enable-pair': ['error', { allowWholeFile: true }],
    'eslint-comments/no-unused-disable': 'error',
    'import/no-relative-packages': 'off',
    'import/extensions': 'off',
    'import/no-extraneous-dependencies': ['error', { devDependencies: true }],
    'import/order': [
      'error',
      {
        groups: [['builtin', 'external', 'internal'], ['parent'], ['sibling', 'index']],
        'newlines-between': 'always',
        alphabetize: { order: 'asc' },
      },
    ],
    'arrow-body-style': 'off',
    'import/prefer-default-export': 'off',
    'no-await-in-loop': 0,
    'no-bitwise': 0,
    'consistent-return': 'off',
    'no-underscore-dangle': 'off',
    'prefer-destructuring': 0,
    'react/display-name': 'off',
    'react/prop-types': 'off',
    'react/react-in-jsx-scope': 'off',
    '@typescript-eslint/no-unused-vars': [
      'error',
      {
        vars: 'all',
        args: 'after-used',
        ignoreRestSiblings: false,
        argsIgnorePattern: '^_',
      },
    ],
  },
  // Disable no-unused-expressions to allow chai 'expect' expressions in testing
  overrides: [
    {
      files: ['*.test.{ts,tsx}', '*/**/__tests__/*.{ts,tsx}'],
      env: {
        jest: true,
      },
      rules: {
        '@typescript-eslint/no-unused-expressions': 'off',
        'import/no-extraneous-dependencies': 'off',
      },
    },
  ],
};
