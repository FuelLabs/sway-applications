import { VecOutput, EscrowInfoOutput } from '@/types/contracts/EscrowAbi';
import type { BigNumberish } from 'fuels';
import { useQuery } from 'react-query';

import { useContract } from './useContract';
import { useWallet } from './useWallet';

export type EscrowInfoWorkaround = EscrowInfoOutput & {
  assets: VecOutput,
}

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
    const temp: EscrowInfoWorkaround = { ...escrow.value[0], assets: escrow.value[1] }
    return temp;
  });
}
