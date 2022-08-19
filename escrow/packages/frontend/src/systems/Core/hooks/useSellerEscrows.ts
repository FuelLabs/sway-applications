import { useQuery } from 'react-query';
import { useWallet } from '../context/AppContext';

import { useContract } from './useContract';

export function useSellerEscrows() {
  const contract = useContract();
  const wallet = useWallet();
  console.log("contract: ", contract);
  const { data: sellerEscrowIds } = useQuery(
    'SellerPage-sellerEscrowIds',
    async () => {
      return contract && contract!.functions.seller_escrows({ Address: { value: wallet?.address! }}).call()
    },
    {
      onSuccess: (data) => console.log("one data: ", data),
    }
  );
  const { data: allEscrows } = useQuery(
    "AllEscrows",
    async () => {
      contract!.functions.escrows().call()
    },
    {
      enabled: !!sellerEscrowIds,
      onSuccess: (data) => console.log("data: ", sellerEscrowIds),
    }
  );
  //console.log("seller ids: ", sellerEscrowIds!.value);
  console.log("all escrows: ", allEscrows);
  const sellerEscrows = allEscrows && sellerEscrowIds!.value.map(escrowId => {
    return allEscrows[parseInt(escrowId.toString())];
  });
  return sellerEscrows;
}