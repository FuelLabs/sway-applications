import { useQuery } from 'react-query';

import { useWallet } from './useWallet';

import { useContract } from './useContract';
import { useEscrows } from './useEscrows';

export function useArbiterEscrows() {
  const contract = useContract();
  const wallet = useWallet();

  const { data: arbiterEscrowIds } = useQuery(
    ['ArbiterPage-arbiterEscrowIds', wallet?.address.toHexString()],
    async () => {
      return (
        contract &&
        wallet &&
        (
          await contract.functions
            .arbiter_escrows({
              Address: {
                value: wallet?.address!.toHexString(),
              },
            })
            .get()
        ).value
      );
    }
  );

  const arbiterEscrows = useEscrows('ArbiterEscrows', arbiterEscrowIds);

  return arbiterEscrows;
}
