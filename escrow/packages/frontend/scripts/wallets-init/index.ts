import { appendFileSync } from "fs";
import { TestUtils, Provider, CoinQuantityLike, BigNumberish, bn, Wallet } from "fuels";

import '../../load.envs';
import { ASSETS, NUM_WALLETS, DECIMAL_PRECISION, FUEL_PROVIDER_URL } from "../../src/config";

async function main() {
    const provider = new Provider(FUEL_PROVIDER_URL);
    // Generate wallets and seed them with a random amount of the configured assets.
    for (let i = 0; i < NUM_WALLETS; i += 1) {
        const nextWallet = await TestUtils.generateTestWallet(provider, ASSETS.map(assetId => {
            const randAssetAmount = bn(Math.floor(Math.random() * 9) + 1);
            let coin: CoinQuantityLike = { assetId, amount: DECIMAL_PRECISION.mul(randAssetAmount) };
            return coin;
        }));
        // Write the private keys to .env for later use by the frontend
        appendFileSync('.env', `VITE_WALLET${i}=${nextWallet.privateKey}\n`);
    }
}

main();