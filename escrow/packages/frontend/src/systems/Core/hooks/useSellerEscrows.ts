import { useQuery } from 'react-query';
import { useWallet } from '../context/AppContext';

import { useContract } from './useContract';

export function useSellerEscrows() {
  const contract = useContract();
  const wallet = useWallet();
  const { data: sellerEscrowIds } = useQuery(
    ['SellerPage-sellerEscrowIds'],
    async () => {
      return contract && (await contract!.functions.seller_escrows({ Address: { value: wallet?.address!.toHexString()! } }).call()).value
    }
  );
  //console.log("seller escrow ids: ", sellerEscrowIds);
  //console.log("bool: ", !!sellerEscrowIds);
  const { data: sellerEscrows } = useQuery(
    ["SellerEscrows"],
    async () => {
      const escrowPromises = sellerEscrowIds!.map(async escrowId => {
        return (await contract!.functions.escrows(escrowId).call()).value
      }
      );
      return await Promise.all(escrowPromises);
    },
    {
      enabled: !!sellerEscrowIds,
      onSuccess: (data) => console.log("data: ", data),
      onError: (err) => console.log(JSON.stringify(err, null, 4)),
    }
  );
  //console.log("seller escrows: ", sellerEscrows);
  return sellerEscrows;
}