import { NFTContractAbi__factory } from "@/contract-types";
import { useWallet } from "@fuels/react";
import { useQuery } from "@tanstack/react-query";
import { CONTRACT_ID } from "@/lib";
import { createAssetId } from "@/utils/assetId";
import { hash } from "fuels";

export const useTotalSupply = (subId: string) => {
    const { wallet, isError, isLoading } = useWallet();

    const query = useQuery({
        queryKey: ["totalSupply", subId],
        queryFn: async () => {
            if (!wallet) throw new Error(`Cannot get total assets if wallet is ${wallet}`);

            const assetId = createAssetId(subId, CONTRACT_ID);

            const contract = NFTContractAbi__factory.connect(CONTRACT_ID, wallet);

            const result = await contract.functions.total_supply(assetId).get();
            return result.value?.toNumber() || 0;
        },
        enabled: !!wallet && !isError && !isLoading,
    });

    return { ...query, totalSupply: query.data };
};
