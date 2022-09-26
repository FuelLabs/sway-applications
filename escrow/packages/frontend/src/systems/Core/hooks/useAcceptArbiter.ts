import { useQueryClient, useMutation } from "react-query";
import toast from "react-hot-toast";
import { useWallet } from "../context/AppContext";
import { contractCheck, updateEscrowQueries } from "../utils/helpers";
import { txFeedback } from "../utils/feedback";
import { useContract } from "./useContract";
import { BigNumberish, bn } from "fuels";

interface UseAcceptArbiterProps { 
    escrowId: BigNumberish;
}

export function useAcceptArbiter({
    escrowId
}: UseAcceptArbiterProps) {
    const queryClient = useQueryClient();
    const wallet = useWallet();
    const contract = useContract();
    const successMsg = "Arbiter accepted successfully.";

    const mutation = useMutation(
        async () => {
            contractCheck(contract);

            const scope = await contract?.functions
                .accept_arbiter(escrowId)
                .txParams({
                    gasPrice: bn(5),
                    gasLimit: 100_000_000,
                    variableOutputs: 1,
                })
                .fundWithRequiredCoins();

            const response = await contract?.wallet?.sendTransaction(scope?.transactionRequest!);
            const result = await response?.waitForResult();

            return result;
        },
        {
            onSuccess: txFeedback(successMsg, handleSuccess),
            onError: handleError,
        }
    );

    function handleSuccess() {
        updateEscrowQueries(queryClient, wallet);
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