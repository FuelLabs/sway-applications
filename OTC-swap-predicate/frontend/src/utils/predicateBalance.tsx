import { BaseWalletLocked, CoinQuantity, Address } from "fuels";



export async function getTokenBalance(address: string) : Promise<{ asset_id: string; amount: string; }[]> {

let addressified = Address.fromAddressOrString(address);

let wallet = new BaseWalletLocked(addressified, "https://node-beta-2.fuel.network/graphql")
let coinQuantities = await wallet.getBalances();

return coinQuantities.map((coinQuantity) => ({"asset_id": coinQuantity.assetId, "amount": coinQuantity.amount.toString()}));

}

