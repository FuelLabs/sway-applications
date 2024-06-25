import { NFTContractAbi__factory } from "@/contract-types";
import { useQuery } from "@tanstack/react-query";
import { CONTRACT_ID, NODE_URL } from "@/lib";
import { createAssetId } from "@/utils/assetId";
import { NFTQueryKeys } from "@/queryKeys";
import { Provider } from "fuels";

export const useTotalSupply = (subId: string) => {

    const query = useQuery({
        queryKey: [NFTQueryKeys.totalSupply, subId],
        queryFn: async () => {
            const provider = await Provider.create(NODE_URL);

            const assetId = createAssetId(subId, CONTRACT_ID);

            const contract = NFTContractAbi__factory.connect(CONTRACT_ID, provider);

            const result = await contract.functions.total_supply(assetId).get();
            return result.value?.toNumber() || 0;
        },
    });

    return { ...query, totalSupply: query.data };
};
