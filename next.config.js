const withMDX = require('@next/mdx')();

/** @type {import('next').NextConfig} */
const nextConfig = withMDX({
  pageExtensions: ['js', 'jsx', 'ts', 'tsx', 'mdx'],
});

module.exports = nextConfig;
