const { config } = require('dotenv');
const { resolve } = require('path');

function getEnvName() {
  if (process.env.NODE_ENV === 'production') {
    return '.env.production';
  }
  if (process.env.NODE_ENV === 'test') {
    return '.env.test';
  }
}

// Load from more specific env file to generic ->
[getEnvName(), '.env'].forEach((envFile) => {
  if (!envFile) return;
  config({
    path: resolve(__dirname, envFile),
  });
});

function getPublicEnvs() {
  const WHITELIST = ['NODE_ENV', 'PUBLIC_URL'];
  return Object.fromEntries(
    Object.entries(process.env).filter(([key]) =>
      WHITELIST.some((k) => k === key || key.match(/^VITE_/))
    )
  );
}

module.exports.getPublicEnvs = getPublicEnvs;
