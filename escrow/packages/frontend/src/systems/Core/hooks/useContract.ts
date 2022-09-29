import { ESCROW_ID } from '../../../config';
import { useWallet } from './useWallet';

import { EscrowAbi__factory } from '../../../types/contracts';

export const useContract = () => {
  const wallet = useWallet();
  return wallet && EscrowAbi__factory.connect(ESCROW_ID, wallet);
};
