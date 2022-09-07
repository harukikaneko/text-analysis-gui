/** @type {import('next').NextConfig} */

const nextConfig = {
  reactStrictMode: true,
  swcMinify: true,
  experimental: {
    images: {
      unoptimized: true,
      allowFutureImage: true,
    },
  },
  compiler: {
    reactRemoveProperties: true,
    emotion: true
  }
};

module.exports = nextConfig;
