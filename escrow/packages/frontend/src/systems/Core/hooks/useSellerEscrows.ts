import { useQuery } from 'react-query';
import { useWallet } from '../context/AppContext';

import { useContract } from './useContract';

export function useSellerEscrows() {
  const contract = useContract();
  const wallet = useWallet();
  console.log("contract: ", contract);
  const {data: sellerEscrowIds } = useQuery('SellerPage-sellerEscrowIds', async () => (await contract!.functions.seller_escrows({ Address: { value: wallet?.address! } }).call()).value);
  const { data: allEscrows } = useQuery("AllEscrows", async () => (await contract!.functions.escrows().call()).value);
  const sellerEscrows = sellerEscrowIds!.map(escrowId => {
    return allEscrows![parseInt(escrowId.toString())];
  });
  return sellerEscrows;
}