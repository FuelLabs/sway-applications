import { useQuery } from 'react-query';

import { useAuctionInfo } from './useAuctionInfo';

import { useContract } from '~/systems/Core/hooks/useContract';

export const useAllAuctionInfo = () => {
  const { contract, isLoading, isError } = useContract();

  const { data: totalAuctions } = useQuery(
    ['totalAuctions'],
    async () => {
      return (await contract?.functions.total_auctions().get())?.value;
    },
    {
      enabled: !isLoading && !isError && !!contract,
    }
  );

  const allAuctionInfo = useAuctionInfo(totalAuctions);
  return allAuctionInfo;
};
