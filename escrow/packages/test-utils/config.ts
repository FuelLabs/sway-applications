import type { Config } from '@jest/types';

export const config: Config.InitialOptions = {
  preset: 'ts-jest/presets/default-esm',
  globals: {
    'ts-jest': {
      useESM: true,
    },
  },
  testTimeout: 20000,
  testEnvironment: 'jsdom',
  testMatch: ['<rootDir>/**/?(*.)+(spec|test).[jt]s?(x)'],
  testPathIgnorePatterns: ['/node_modules/', '/dist/', '/cypress'],
  modulePathIgnorePatterns: ['<rootDir>/dist/'],
  reporters: ['default', 'github-actions'],
  setupFiles: ['dotenv/config'],
  setupFilesAfterEnv: ['@escrow/test-utils/setup.ts'],
  collectCoverageFrom: [
    '<rootDir>/src/**/*.{ts,tsx}',
    '!<rootDir>/src/**/*d.ts',
    '!<rootDir>/src/**/*test.{ts,tsx}',
    '!<rootDir>/src/**/test-*.{ts}',
    '!<rootDir>/src/**/__mocks__/**',
    '!<rootDir>/src/types/**',
  ],
  moduleNameMapper: {
    '.+\\.(css|scss|png|jpg|svg)$': 'jest-transform-stub',
    '~/(.*)$': '<rootDir>/src/$1',
    '^(\\.{1,2}/.*)\\.js$': '$1',
  },
};

export default config;
