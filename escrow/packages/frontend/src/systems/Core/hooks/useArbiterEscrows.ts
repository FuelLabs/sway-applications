import { useQuery } from 'react-query';

import { useWallet } from './useWallet';

import { useContract } from './useContract';
import { useEscrows } from './useEscrows';
import { bn } from 'fuels';

export function useArbiterEscrows() {
  const contract = useContract();
  const wallet = useWallet();

  let { data: arbiterEscrowIds } = useQuery(
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

  return { arbiterEscrows, arbiterEscrowIds: !arbiterEscrowIds ? [bn(0)] : arbiterEscrowIds};
}
