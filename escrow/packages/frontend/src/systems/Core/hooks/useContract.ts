import { ESCROW_ID } from '../../../config';
import { EscrowAbi__factory } from '../../../types/contracts';

import { useWallet } from './useWallet';

export const useContract = () => {
  const wallet = useWallet();
  return wallet && EscrowAbi__factory.connect(ESCROW_ID, wallet);
};
