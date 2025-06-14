# Stylus by Example

This is a [Next.js](https://nextjs.org/) project bootstrapped with [`create-next-app`](https://github.com/vercel/next.js/tree/canary/packages/create-next-app), showcasing Stylus smart contract examples.

## 📋 Prerequisites

> **📝 Important:** This repository uses Git submodules for external examples. After cloning, you need to initialize the submodules.

## Getting Started

### 1. Clone the Repository

**Clone with submodules (recommended):**
```bash
git clone --recursive https://github.com/OffchainLabs/stylus-by-example.git
cd stylus-by-example
```

**Or clone normally, then initialize submodules:**
```bash
git clone https://github.com/OffchainLabs/stylus-by-example.git
cd stylus-by-example
git submodule update --init --recursive
```

### 2. Install Dependencies and Run

```bash
pnpm install
pnpm dev
```

Open [http://localhost:3000](http://localhost:3000) with your browser to see the result.

## 📁 Repository Structure

```
example_code/
├── internal/    # Internal examples and applications
└── external/    # Examples from stylus-sdk-rs (via submodule)
```

- **Internal examples**: Custom examples and applications specific to this project
- **External examples**: Latest examples from the [stylus-sdk-rs](https://github.com/OffchainLabs/stylus-sdk-rs) repository

## 🔄 Updating External Examples

The external examples are automatically updated via GitHub Actions, but you can manually update them:

```bash
git submodule update --remote
git add .
git commit -m "Update external examples"
```

## Development

You can start editing the page by modifying `app/page.tsx`. The page auto-updates as you edit the file.

This project uses [`next/font`](https://nextjs.org/docs/basic-features/font-optimization) to automatically optimize and load Inter, a custom Google Font.

## Learn More

### Next.js Resources
- [Next.js Documentation](https://nextjs.org/docs) - learn about Next.js features and API.
- [Learn Next.js](https://nextjs.org/learn) - an interactive Next.js tutorial.
- [Next.js GitHub repository](https://github.com/vercel/next.js/) - your feedback and contributions are welcome!

### Stylus Resources
If you'd like to know more about Stylus, feel free to take a look at [Arbitrum's Stylus technical pages](https://docs.arbitrum.io/stylus/stylus-gentle-introduction).

## Deploy on Vercel

The easiest way to deploy your Next.js app is to use the [Vercel Platform](https://vercel.com/new?utm_medium=default-template&filter=next.js&utm_source=create-next-app&utm_campaign=create-next-app-readme) from the creators of Next.js.

Check out our [Next.js deployment documentation](https://nextjs.org/docs/deployment) for more details.
