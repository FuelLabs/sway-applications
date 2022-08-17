import { useQuery } from 'react-query';
import { useWallet } from '../context/AppContext';

import { useContract } from './useContract';

export function useSellerEscrows() {
  const contract = useContract();
  const wallet = useWallet();
  return useQuery('SellerPage-sellerEscrows', async () => (await contract!.functions.seller_escrows({ Address: { value: wallet?.address! } }).call()).value);
}