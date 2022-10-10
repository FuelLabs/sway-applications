import { bn } from 'fuels';
import { useQuery } from 'react-query';

import { useContract } from './useContract';
import { useEscrows } from './useEscrows';
import { useWallet } from './useWallet';

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

  return { sellerEscrows, sellerEscrowIds: !sellerEscrowIds ? [bn(0)] : sellerEscrowIds };
}
