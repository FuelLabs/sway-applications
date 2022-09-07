import { useMutation, useQueryClient } from "react-query";
import toast from "react-hot-toast";
import { txFeedback } from "../utils/feedback";
import { useContract } from "./useContract";

interface UseDisputeProps {
    escrowId: bigint;
}

export function useDispute({
    escrowId
}: UseDisputeProps) {
    const queryClient = useQueryClient();
    const contract = useContract();
    const successMsg = "Dispute successful.";

    const mutation = useMutation(
        async () => {
            if (!contract) {
                throw new Error("Contract not found");
            }

            const scope = await contract.functions
                .dispute(escrowId)
                .txParams({
                    gasPrice: BigInt(5),
                    bytePrice: BigInt(5),
                    gasLimit: 100_000_000,
                })
                .fundWithRequiredCoins();

            const response = await contract.wallet?.sendTransaction(scope.transactionRequest);
            const result = await response?.waitForResult();

            return result;
        },
        {
            onSuccess: txFeedback(successMsg, handleSuccess),
            onError: handleError,
        }
    );

    function handleSuccess() {
        queryClient.fetchQuery(["BuyerEscrows"]);
        queryClient.fetchQuery(["SellerEscrows"]);
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