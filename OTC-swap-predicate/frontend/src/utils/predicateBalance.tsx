import { Address, BaseWalletLocked, CoinQuantity } from "fuels";

async function getTokenBalance(address: Address, network: string) : Promise<CoinQuantity[]> {
    let wallet = new BaseWalletLocked(address, network)
    let coinQuantities = await wallet.getBalances();
    return coinQuantities;
}

export default getTokenBalance;
