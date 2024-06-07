import { NFTContractAbi__factory } from "@/contract-types";
import { useWallet } from "@fuels/react";
import { useQuery } from "@tanstack/react-query";

export const useTotalAssets = (contractId: string) => {
    const { wallet, isError, isLoading } = useWallet();

    const query = useQuery({
        queryKey: ["totalAssets", contractId],
        queryFn: async () => {
            if (!wallet) throw new Error(`Cannot get total assets if wallet is ${wallet}`);

            const contract = NFTContractAbi__factory.connect(contractId, wallet);

            const result = await contract.functions.total_assets().get();
            return result.value;
        },
        enabled: !!wallet && !isError && !isLoading,
    });

    return { ...query, totalAssets: query.data };
};
