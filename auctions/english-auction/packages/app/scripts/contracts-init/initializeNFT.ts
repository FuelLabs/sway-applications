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
  const buyerWallet = walletManager.getWallet(accounts[1].address);

  const sellerNFTContract = NFTAbi__factory.connect(process.env.VITE_NFT_ID!, sellerWallet);
  const buyerNFTContract = NFTAbi__factory.connect(process.env.VITE_NFT_ID!, buyerWallet);

  await nftContract.functions
    .mint(bn(1), {
      Address: { value: '0x80d5e88c2b23ec2be6b2e76f3499a1a2755bb2773363785111a719513fb57b8e' },
    })
    .txParams({ gasPrice: 1 })
    .call();

  const ownerOf = (await sellerNFTContract.functions.owner_of(bn(0)).get()).value;
  console.log('owner: ', ownerOf);

  await sellerNFTContract.functions
    .approve({ ContractId: { value: CONTRACT_ID } }, bn(0))
    .txParams({ gasPrice: 1 })
    .call();

  const approved = (await sellerNFTContract.functions.approved(bn(0)).get()).value;
  console.log('approved: ', approved);

  await nftContract.functions
    .mint(bn(1), {
      Address: { value: '0xf13c949256d0e119fecaec414ea452f21f9dc1870fb6262ff53b37c32cab4749' },
    })
    .txParams({ gasPrice: 1 })
    .call();

  const ownerOf2 = (await buyerNFTContract.functions.owner_of(bn(1)).get()).value;
  console.log('owner: ', ownerOf2);

  await buyerNFTContract.functions
    .approve({ ContractId: { value: CONTRACT_ID } }, bn(1))
    .txParams({ gasPrice: 1 })
    .call();

  const approved2 = (await buyerNFTContract.functions.approved(bn(1)).get()).value;
  console.log('approved: ', approved2);
}
