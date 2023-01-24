import { WalletManager } from '@fuel-ts/wallet-manager';
import { Wallet, bn } from 'fuels';

import { CONTRACT_ID } from '../../src/config';
import { NFTAbi__factory } from '../../src/types/contracts';

export async function initializeNFT() {
  const wallet = Wallet.fromPrivateKey(process.env.WALLET_SECRET!, process.env.PROVIDER_URL);
  const nftContract = NFTAbi__factory.connect(process.env.VITE_NFT_ID!, wallet);

  const config = { type: 'mnemonic', secret: process.env.WALLET_MNEMONIC! };
  const walletManager = new WalletManager();
  await walletManager.unlock('123123123');
  await walletManager.addVault(config);
  await walletManager.addAccount();
  const accounts = walletManager.getAccounts();
  const sellerWallet = walletManager.getWallet(accounts[0].address);
  sellerWallet.connect(process.env.PROVIDER_URL!);
  const buyerWallet = walletManager.getWallet(accounts[1].address);
  buyerWallet.connect(process.env.PROVIDER_URL!);

  const sellerNFTContract = NFTAbi__factory.connect(process.env.VITE_NFT_ID!, sellerWallet);
  const buyerNFTContract = NFTAbi__factory.connect(process.env.VITE_NFT_ID!, buyerWallet);

  const sellerAddress = '0x80d5e88c2b23ec2be6b2e76f3499a1a2755bb2773363785111a719513fb57b8e';
  const sellerMintCount = bn(10);

  await nftContract.functions
    .mint(sellerMintCount, {
      Address: { value: sellerAddress },
    })
    .txParams({ gasPrice: 1 })
    .call();

  console.log(`Minted NFTs with token ids [0-9] to address: ${sellerAddress}`);

  for (let i = 0; i < sellerMintCount.toNumber(); i += 1) {
    await sellerNFTContract.functions
      .approve({ ContractId: { value: CONTRACT_ID } }, bn(i))
      .txParams({ gasPrice: 1 })
      .call();
  }

  console.log(`Approved identity ${CONTRACT_ID} to transfer token ids 0-9`);

  const buyerAddress = '0xf13c949256d0e119fecaec414ea452f21f9dc1870fb6262ff53b37c32cab4749';
  const buyerMintCount = bn(10);

  await nftContract.functions
    .mint(buyerMintCount, {
      Address: { value: buyerAddress },
    })
    .txParams({ gasPrice: 1 })
    .call();

  console.log(`Minted NFTs with token ids [10-19] to address: ${buyerAddress}`);

  for (let i = 0; i < buyerMintCount.toNumber(); i += 1) {
    await buyerNFTContract.functions
      .approve({ ContractId: { value: CONTRACT_ID } }, bn(i + sellerMintCount.toNumber()))
      .txParams({ gasPrice: 1 })
      .call();
  }
  console.log(`Approved identity ${CONTRACT_ID} to transfer token ids 10-19`);
}
