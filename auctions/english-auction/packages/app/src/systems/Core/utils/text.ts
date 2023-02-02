import { NativeAssetId } from 'fuels';

export const getTokenText = (assetId: string) => {
  return assetId === NativeAssetId ? 'ETH' : 'Token';
};

export const getAssetText = (isNFT: boolean = false, assetId: string = NativeAssetId) => {
  if (isNFT) {
    return 'NFT';
  }
  const text = getTokenText(assetId);
  return text;
};

export const getSlicedAddress = (address: string) => {
  return `${address.slice(0, 4)}...${address.slice(-4)}`;
};
