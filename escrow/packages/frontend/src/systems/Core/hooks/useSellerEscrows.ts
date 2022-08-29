import { useEffect, useState } from 'react';
import { useQuery } from 'react-query';
import { useWallet } from '../context/AppContext';

import { useContract } from './useContract';

export function useSellerEscrows() {
  const [result, setResult] = useState<any[] | undefined>();
  const [loading, setLoading] = useState("false");
  const contract = useContract();
  const wallet = useWallet();
  useEffect(() => {
    async function fetchSellerEscrows() {
      try {
        setLoading("true");
        console.log("contract: ", contract);
        if (!contract) {
          setResult(undefined);
          return;
        }
        // const { data: sellerEscrowIds } = useQuery(
        //   ['SellerPage-sellerEscrowIds', contract],
        //   async () => {
        //     return contract && (await contract!.functions.seller_escrows({ Address: { value: wallet?.address!.toHexString()! } }).call()).value
        //   },
        //   {
        //     onSuccess: (data) => console.log("one data: ", data),
        //   }
        // );
        const sellerEscrowIds = contract && (await contract!.functions.seller_escrows({ Address: { value: wallet?.address!.toHexString()! } }).call()).value;
        console.log("seller escrow ids: ", sellerEscrowIds);
        console.log("bool: ", !!sellerEscrowIds);
        // const { data: allEscrows } = useQuery(
        //   ["AllEscrows", contract],
        //   async () => {
        //     return (await contract!.functions.escrows().call()).value
        //   },
        //   {
        //     enabled: !!sellerEscrowIds,
        //     onSuccess: (data) => console.log("data: ", sellerEscrowIds),
        //     onError: (err) => console.log(JSON.stringify(err, null, 4)),
        //   }
        // );
        const allEscrows = sellerEscrowIds && (await contract!.functions.escrows().call()).value;
        console.log("all escrows: ", allEscrows);
        const sellerEscrows = allEscrows && sellerEscrowIds!.map(escrowId => {
          return allEscrows[(parseInt(escrowId.toString()))];
        });
         setResult(sellerEscrows!);
      } catch (err) {
        console.log("err", err);
        setLoading("null");
      }
    }
    fetchSellerEscrows()
  });

  return { result, loading }
}