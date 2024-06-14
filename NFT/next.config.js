/** @type {import('next').NextConfig} */
const nextConfig = {
  reactStrictMode: true,
  transpilePackages: ['tictactoe'],
  experimental: {
    externalDir: true,
  }
};

module.exports = nextConfig;
