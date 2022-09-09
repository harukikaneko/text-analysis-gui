/** @type {import('next').NextConfig} */

const nextConfig = {
  reactStrictMode: true,
  swcMinify: true,
  images: {
    unoptimized: true
  },
  compiler: {
    reactRemoveProperties: true,
    emotion: true
  }
};

module.exports = nextConfig;
