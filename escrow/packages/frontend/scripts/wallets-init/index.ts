import { appendFileSync } from "fs";
import { TestUtils, Provider, toBigInt } from "fuels";

import '../../load.envs';
import { ASSETS, DECIMAL_PRECISION, FUEL_PROVIDER_URL } from "../../src/config";

// Initial number of wallets to populate in app
const NUM_WALLETS = 10;

async function main() {
    const provider = new Provider(FUEL_PROVIDER_URL);
    // Generate wallets and seed them with a random amount of the configured assets.
    for (let i = 0; i < NUM_WALLETS; i += 1) {
        const nextWallet = await TestUtils.generateTestWallet(provider, ASSETS.map(assetId =>  {
            const randAssetAmount = Math.floor(Math.random() * 9) + 1;
            return { assetId, amount: DECIMAL_PRECISION * toBigInt(randAssetAmount) }
        }));
        // Write the private keys to .env for later use by the frontend
        appendFileSync('.env', `WALLET${i}=${nextWallet.privateKey}\n`);
    }
}

main();