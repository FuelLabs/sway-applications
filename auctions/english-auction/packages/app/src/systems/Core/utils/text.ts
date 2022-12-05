import { NativeAssetId } from "fuels";

export const getTokenText = (assetId: string) => {
    return assetId === NativeAssetId ? "ETH" : "Token";
};

export const getAssetText = (
    isNFT: boolean = false,
    assetId: string | undefined = NativeAssetId
) => {
    if (isNFT) {
        return "NFT";
    }
    const text = getTokenText(assetId!);
    return text;
};