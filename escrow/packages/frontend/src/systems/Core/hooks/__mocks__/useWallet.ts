import { Wallet } from 'fuels';

import { FUEL_PROVIDER_URL } from '../../../../config';
import * as useWallet from '../useWallet';
import * as useWalletList from '../useWalletList';

export function createWallet() {
  return Wallet.generate({ provider: FUEL_PROVIDER_URL });
}

export function mockUseWalletList(walletList: Wallet[]) {
  return jest.spyOn(useWalletList, 'useWalletList').mockImplementation(() => walletList);
}

export function mockUseWallet(wallet: Wallet) {
  return jest.spyOn(useWallet, 'useWallet').mockImplementation(() => wallet);
}
