import { BaseWalletLocked, Address } from "fuels";

export async function getTokenBalance(address: string, providerAddress: string) : Promise<{ asset_id: string; amount: string; }[]> {
    let addressified = Address.fromAddressOrString(address);

    let wallet = new BaseWalletLocked(addressified, providerAddress)
    let coinQuantities = await wallet.getBalances();

    return coinQuantities.map((coinQuantity) => (
        {"asset_id": coinQuantity.assetId, "amount": coinQuantity.amount.toString()}
    ));
}
