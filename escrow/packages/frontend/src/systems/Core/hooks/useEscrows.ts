import type { BigNumberish } from 'fuels';
import { useQuery } from 'react-query';

import { useWallet } from '../context/AppContext';

import { useContract } from './useContract';

export function useEscrows(queryString: string, escrowIds: BigNumberish[] | null | undefined) {
  const contract = useContract();
  const wallet = useWallet();

  // We have to convert the bigints to strings bc bigints are not serializable
  const { data: escrows } = useQuery(
    [queryString, wallet?.address.toHexString()],
    async () => {
      const escrowPromises = escrowIds!.map((escrowId) => {
        return contract!.functions.escrows(escrowId).get();
      });
      return Promise.all(escrowPromises);
    },
    {
      enabled: !!escrowIds,
    }
  );

  return escrows?.map((escrow) => {
    return escrow.value;
  });
}
