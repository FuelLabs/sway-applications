import { Address, BaseWalletLocked, CoinQuantity } from "fuels";

async function getTokenBalance(address: Address, provider: string) : Promise<CoinQuantity[]> {
    let wallet = new BaseWalletLocked(address, provider)
    let coinQuantities = await wallet.getBalances();

    return coinQuantities;
}

export default getTokenBalance;
