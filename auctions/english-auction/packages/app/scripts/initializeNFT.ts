import { NativeAssetId, Wallet, bn } from 'fuels';

import { NFTContractAbi__factory } from '../src/types/contracts';

export async function initializeNFT() {
  const wallet = Wallet.fromPrivateKey(
    process.env.VITE_WALLET_SECRET!,
    process.env.VITE_PROVIDER_URL
  );
  const nftContract = NFTContractAbi__factory.connect(process.env.VITE_NFT_ID!, wallet);
  await nftContract.functions
    .constructor(false, { Address: { value: NativeAssetId } }, bn(1000))
    .call();
  await nftContract.functions
    .mint(bn(100), {
      Address: { value: '0xf38c5f68fe92a98fa38e5381096a64b68b87d66eb38a50a414c814232809940f' },
    })
    .call();
  await nftContract.functions
    .mint(bn(100), {
      Address: { value: '0x352406edf3f532cce7570c29f49c2a7d99d8ff5669bcf20a238cff26425c2110' },
    })
    .call();
}
