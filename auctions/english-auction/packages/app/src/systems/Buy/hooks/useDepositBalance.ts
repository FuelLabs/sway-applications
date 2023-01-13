import type { BN } from 'fuels';
import { useQuery } from 'react-query';

import { useContract } from '~/systems/Core/hooks/useContract';
import type { IdentityInput } from '~/types/contracts/EnglishAuctionAbi';

export const useDepositsBalance = (auctionId: BN, identity: IdentityInput) => {
  const { contract, isLoading, isError } = useContract();

  const { data: balance } = useQuery(
    ['depositBalance'],
    async () => {
      const depositBalance = (await contract?.functions.deposit_balance(auctionId, identity).get())
        ?.value;
      return depositBalance;
    },
    {
      enabled: !isLoading && !isError && !!contract,
    }
  );

  return balance;
};
