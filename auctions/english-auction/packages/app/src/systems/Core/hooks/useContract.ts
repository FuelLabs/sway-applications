import { CONTRACT_ID } from '~/config';
import { EnglishAuctionAbi__factory } from '~/types/contracts';

export const useContract = () => {
  return EnglishAuctionAbi__factory.connect(CONTRACT_ID, window.FuelWeb3.getProvider());
};
