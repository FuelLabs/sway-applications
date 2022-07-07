<h1>⚡️ Fuels UI - Escrow Application</h1>

<h2>📝&nbsp; Table of Contents</h2>

- [🙋🏻&nbsp; Getting Started](#-getting-started)
- [🖥️ Running The Prokect](#running-the-project)
- [📜&nbsp; License](#-license)

---

## 🙋🏻&nbsp; Getting Started

First, go through the setup process in [CONTRIBUTING.md](../../../CONTRIBUTING.md)

## 🖥️ Running The Project

In the root directory make sure you start your local fuel node:

```bash
docker compose up
```

Then, inside of `/frontend` build and deploy the contracts

```bash
pnpm build-contracts
pnpm deploy-contracts
```

Then run the development server:

```bash
pnpm run dev
# or
yarn dev
```

Open [http://localhost:3000](http://localhost:3000) with your browser to see the result.

You can start editing the page by modifying any `pages/` folder inside the `src/systems/`.
The page auto-updates as you edit the file.

## Testing the Program

Insde of `/contracts/escrow` folder test the contracts:

```bash
forc test
```

## 📜&nbsp; License

The primary license for this repo is `Apache 2.0`, see [`LICENSE`](./LICENSE).
