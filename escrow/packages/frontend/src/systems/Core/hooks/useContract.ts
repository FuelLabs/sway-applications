import { useWallet } from "../context/AppContext";

import { ESCROW_ID } from '../../../config';
import { EscrowAbi__factory } from "@/types/contracts";

export const useContract = () => {
  const wallet = useWallet();
  return wallet && EscrowAbi__factory.connect(ESCROW_ID, wallet);
};