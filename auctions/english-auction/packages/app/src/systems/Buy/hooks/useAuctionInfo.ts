import { useQuery } from "react-query";
import { BN, bn } from "fuels";

import { useContract } from "~/systems/Core/hooks/useContract";

export const useAuctionInfo = (totalAuctions: BN | undefined) => {
    const contract = useContract();

    const { data: auctionInfo } = useQuery(
        ['auctionInfo'],
        async () => {
            let auctionInfoPromises = [];
            for (let auctionId = bn(0); auctionId.lt(totalAuctions!); auctionId = auctionId.add(1)) {
                auctionInfoPromises.push(contract?.functions.auction_info(auctionId).get());
            }
            return (await Promise.all(auctionInfoPromises!));
        },
        { enabled: !!contract && !!totalAuctions }
    );

    return auctionInfo?.map((auction) => {
        return auction?.value;
    });
};