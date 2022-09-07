import { useAtomValue } from "jotai";
import { useMutation, useQueryClient } from "react-query";
import toast from "react-hot-toast";
import { walletIndexAtom } from "../jotai";
import { txFeedback } from "../utils/feedback";
import { useContract } from "./useContract";

interface UseTransferToSellerProps {
    escrowId: bigint;
}

export function useTransferToSeller({ escrowId }: UseTransferToSellerProps) {
    const queryClient = useQueryClient();
    const walletIdx = useAtomValue(walletIndexAtom);
    const contract = useContract();
    const successMsg = "Transfer to seller successful.";

    const mutation = useMutation(
        async () => {
            if (!contract) {
                throw new Error("Contract not found");
            }

            const scope = await contract!.functions
                .transfer_to_seller(escrowId)
                .txParams({
                    gasPrice: BigInt(5),
                    bytePrice: BigInt(5),
                    gasLimit: 100_000_000,
                    variableOutputs: 3,
                })
                .fundWithRequiredCoins();

            const response = await contract!.wallet?.sendTransaction(scope.transactionRequest);
            const result = await response?.waitForResult();

            return result;
        },
        {
            onSuccess: txFeedback(successMsg, handleSuccess),
            onError: handleError,
        }
    );

    function handleSuccess() {
        // Trigger queries to update components
        queryClient.fetchQuery(["SellerEscrows"]);
        queryClient.fetchQuery(["BuyerEscrows"]);
        queryClient.fetchQuery(["ArbiterEscrows"]);
    }

    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    function handleError(e: any) {
        const errors = e?.response?.errors;

        if (errors?.length) {
            if (errors[0].message === 'enough coins could not be found') {
                toast.error('Not enough balance in your wallet to deposit');
            } else {
                toast.error(errors[0].message);
            }
        } else {
            toast.error('Error when trying to deposit');
        }
    }

    return mutation;
}