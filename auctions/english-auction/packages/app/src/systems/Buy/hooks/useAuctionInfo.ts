import type { BN } from 'fuels';
import { bn } from 'fuels';
import { useQuery } from 'react-query';

import { useContract } from '~/systems/Core/hooks/useContract';

export const useAuctionInfo = (totalAuctions: BN | undefined) => {
  const contract = useContract();

  const { data: auctionInfo } = useQuery(
    ['auctionInfo'],
    async () => {
      const auctionInfoPromises = [];
      for (let auctionId = bn(0); auctionId.lt(totalAuctions!); auctionId = auctionId.add(1)) {
        auctionInfoPromises.push(contract?.functions.auction_info(auctionId).get());
      }
      return Promise.all(auctionInfoPromises!);
    },
    { enabled: !!contract && !!totalAuctions }
  );

  return auctionInfo?.map((auction) => {
    return auction?.value;
  });
};
