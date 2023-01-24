## Project Structure

- [/public](./public/) contains publicly accessible assets;
- [/src](./src/) contains frontend code (assets, components, hooks, etc).

### Environment variables

| name                   | description                                                                             |
| ---------------------- | --------------------------------------------------------------------------------------- |
| VITE_FUEL_PROVIDER_URL | Fuel-core network url normally set as `http://localhost:4001` for development           |
| VITE_CONTRACT_ID       | English auction contract id. This is automatically set by the `english-auction-scripts` |
| VITE_TOKEN_ID          | Token contract id this is automatically set by the `english-auction-scripts`            |
| VITE_NFT_ID            | NFT contract id. This is automatically set by `english-auction-scripts`                 |
