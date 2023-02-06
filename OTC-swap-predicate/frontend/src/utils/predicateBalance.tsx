import { BaseWalletLocked, Address, CoinQuantity } from "fuels";

export async function getTokenBalance(address: Address, providerAddress: string) : Promise<CoinQuantity[]> {
    let wallet = new BaseWalletLocked(address, providerAddress)
    let coinQuantities = await wallet.getBalances();

    return coinQuantities;
}
