## Getting Started

> ⚠️ If is your first time running the project you should start [here](../../README.md)

Install all dependencies with `pnpm`:

```bash
pnpm install
```

This command also copies the contents of `.env.example` to a newly created `.env` file which the frontend will use to interact with your deployed contracts.  Before starting the development server make sure the contract id and token id environment varialbes are set to the corresponding contract addresses.

Then run the development server:

```bash
pnpm dev
```

Open [http://localhost:3000](http://localhost:3000) in your browser to see the result.

You can start editing the pages by modifying `src/systems/Core/pages`.   The page auto-updates as you edit the file.

## Project Structure

- [/public](./public/) contains publicly accessible assets
- [/src](./src/) contains frontend code (components, hooks, etc)
- [/scripts](./scripts/) contains scripts to initialize wallets

## Environment Variables

| name                      | description                                                                   |
| ------------------------- | ----------------------------------------------------------------------------- |
| VITE_FUEL_PROVIDER_URL    | Fuel-core network url normally set as `http://localhost:4000` for development |
| VITE_FUEL_FAUCET_URL      | (Unused!) Faucet API url normally set as `http://localhost:4040` for development        |
| VITE_FAUCET_RECAPTCHA_KEY | (Unused!) Recaptcha key used only on live environment                                   |
| VITE_CONTRACT_ID          | Escrow contract id this is automatically set by the `escrow-scripts`      |
| VITE_TOKEN_ID             | Token contract id this is automatically set by the `escrow-scripts`         |