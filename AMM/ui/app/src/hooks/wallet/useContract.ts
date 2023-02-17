import { useQuery } from "@tanstack/react-query";
import { useWallet } from "./useWallet";

export const useContract = (contractId: string, factory: any) => {
    const { wallet, isLoading, isError } = useWallet();

    const {
        data: contract,
        isLoading: isContractLoading,
        isError: isContractError,
    } = useQuery(
        ["contract", wallet],
        () => {
            return factory.connect(contractId, wallet!);
        },
        {
            enabled: !isLoading && !isError && !!wallet,
        }
    );

    return { contract, isLoading: isContractLoading, isError: isContractError };
};
