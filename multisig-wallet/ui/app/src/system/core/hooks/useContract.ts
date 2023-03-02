import { useQuery } from "@tanstack/react-query";
import { useWallet } from "./useWallet";
import { MultisigContractAbi__factory } from "../../../contracts";

export const useContract = () => {
    const { wallet, isLoading, isError } = useWallet();
    const CONTRACT_ID = "0xde841dfb22ce3155150e06c8a4b79ca94aa11e64ddac7e60467cd1a364b890e4";

    const {
        data: contract,
        isLoading: isContractLoading,
        isError: isContractError,
    } = useQuery(
        ["contract", wallet],
        () => {
            return MultisigContractAbi__factory.connect(CONTRACT_ID, wallet!);
        },
        {
            enabled: !isLoading && !isError && !!wallet,
        }
    );

    return { contract, isLoading: isContractLoading, isError: isContractError };
};
