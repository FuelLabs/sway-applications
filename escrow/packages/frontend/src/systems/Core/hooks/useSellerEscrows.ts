import { useQuery } from 'react-query';

import { useWallet } from '../context/AppContext';

import { useContract } from './useContract';
import { useEscrows } from './useEscrows';

export function useSellerEscrows() {
  const contract = useContract();
  const wallet = useWallet();
  const { data: sellerEscrowIds } = useQuery(
    ['SellerPage-sellerEscrowIds', wallet?.address.toHexString()],
    async () => {
      return (
        contract &&
        wallet &&
        (
          await contract!.functions
            .seller_escrows({ Address: { value: wallet.address!.toHexString() } })
            .get()
        ).value
      );
    }
  );

  const sellerEscrows = useEscrows('SellerEscrows', sellerEscrowIds);

  return sellerEscrows;
}
