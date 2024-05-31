import { NFTContractAbi__factory } from "@/contract-types";
import { CONTRACT_ID } from "@/lib";
import { useWallet } from "@fuels/react";
import { useQuery } from "@tanstack/react-query";

export const useTotalAssets = () => {
    const { wallet, isError, isLoading } = useWallet();

    const query = useQuery({
        queryKey: ["totalAssets"],
        queryFn: async () => {
            if (!wallet) throw new Error(`Cannot get total assets if wallet is ${wallet}`);

            const contract = NFTContractAbi__factory.connect(CONTRACT_ID, wallet);

            const result = await contract.functions.total_assets().get();
            return result.value;
        },
        enabled: !!wallet && !isError && !isLoading,
    });

    return { ...query, totalAssets: query.data };
};
