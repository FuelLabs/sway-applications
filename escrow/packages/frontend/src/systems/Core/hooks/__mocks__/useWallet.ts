import { FUEL_PROVIDER_URL } from "../../../../config";
import { Wallet } from "fuels";
import * as useWalletList from "../../context/AppContext";

export function createWallet() {
    return Wallet.generate({ provider: FUEL_PROVIDER_URL });
}

export function mockUseWalletList(walletList: Wallet[]) {
    return jest.spyOn(useWalletList, "useWalletList").mockImplementation(() => walletList);
}