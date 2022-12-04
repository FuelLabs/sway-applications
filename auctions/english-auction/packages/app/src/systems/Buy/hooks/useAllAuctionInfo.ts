import { useQuery } from "react-query";

import { useContract } from "~/systems/Core/hooks/useContract";
import { useAuctionInfo } from "./useAuctionInfo";

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