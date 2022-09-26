import { useWalletList } from "../../context/AppContext";

export function mockUseWallet() {
    return jest.spyOn(useWalletList, "useWalletList").mockImplementation(() => );
}