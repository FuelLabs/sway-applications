import type { BigNumberish } from 'fuels';
import { useQuery } from 'react-query';

import { useContract } from './useContract';
import { useWallet } from './useWallet';

export function useArbiterProposal(escrowId: BigNumberish) {
  const wallet = useWallet();
  const contract = useContract();

  const { data: arbiterProposal } = useQuery(
    ['ArbiterProposal', wallet?.address.toHexString(), escrowId.toString()],
    async () => {
      return contract && (await contract!.functions.arbiter_proposals(escrowId).get()).value;
    }
  );

  return arbiterProposal;
}
