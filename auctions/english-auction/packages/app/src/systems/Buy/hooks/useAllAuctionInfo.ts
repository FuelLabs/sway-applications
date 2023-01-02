import { useQuery } from 'react-query';

import { useAuctionInfo } from './useAuctionInfo';

import { useContract } from '~/systems/Core/hooks/useContract';

export const useAllAuctionInfo = () => {
  const contracts = useContract();

  const { data: totalAuctions } = useQuery(
    ['totalAuctions'],
    async () => {
      return (await contracts?.functions.total_auctions().get())?.value;
    },
    {
      enabled: !!contracts,
    }
  );

  const allAuctionInfo = useAuctionInfo(totalAuctions);
  return allAuctionInfo;
};
