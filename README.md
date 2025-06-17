# Stylus by Example

This is a [Next.js](https://nextjs.org/) project bootstrapped with [`create-next-app`](https://github.com/vercel/next.js/tree/canary/packages/create-next-app), showcasing Stylus smart contract examples.

## 🚀 Quick Start

```bash
git clone --recursive https://github.com/OffchainLabs/stylus-by-example.git
cd stylus-by-example
pnpm install
pnpm dev
```

Open [http://localhost:3000](http://localhost:3000) with your browser to see the result.

> **📝 Note:** If you forgot `--recursive`, run `git submodule update --init --recursive`

## 📁 Repository Structure

```
example_code/
├── internal/    # Custom examples and applications
└── external/    # Examples from stylus-sdk-rs (auto-updated daily)
```

- **Internal examples**: Custom examples and applications specific to this project
- **External examples**: Latest examples from the [stylus-sdk-rs](https://github.com/OffchainLabs/stylus-sdk-rs) repository

## 🔄 How Examples Stay Updated

- **🤖 Automatic**: External examples update daily via GitHub Actions
- **📋 Perfect compatibility**: Examples use path dependencies to submodule for zero version conflicts
- **✨ Always current**: You get the latest working examples from the Stylus SDK

When new examples or fixes are available, you'll see PRs like "Auto-update external examples" that can be reviewed and merged.

## 🛠️ For Contributors

If you're contributing to this project and want the absolute latest examples:

```bash
# Optional: Get bleeding-edge examples (they auto-update daily anyway)
./scripts/sync-external-examples.sh
```

For testing changes to the documentation system or FileCodeBlock components.

## Development

You can start editing the page by modifying `app/page.tsx`. The page auto-updates as you edit the file.

This project uses [`next/font`](https://nextjs.org/docs/basic-features/font-optimization) to automatically optimize and load Inter, a custom Google Font.

## 📖 Code Import System

This documentation automatically imports code from the `example_code/` directory:

- **Live sync**: Code blocks show actual files from the repository
- **Always accurate**: Documentation stays in sync with working examples
- **Zero maintenance**: No manual copy-pasting of code needed

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

## 🤝 Contributing

When contributing to this project:

1. **Regular contributions**: Just clone and start coding - examples are already up-to-date
2. **Testing with latest examples**: Run `./scripts/sync-external-examples.sh` if needed
3. **Adding new examples**: Add them to `example_code/internal/` 
4. **Updating external examples**: They update automatically, but you can manually sync for testing

The external examples use the same workspace dependencies as the original Stylus SDK, ensuring perfect compatibility and all features work out of the box.