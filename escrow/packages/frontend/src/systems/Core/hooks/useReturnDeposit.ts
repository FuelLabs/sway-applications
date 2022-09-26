import { useAtomValue } from "jotai";
import { useMutation, useQueryClient } from "react-query";
import toast from "react-hot-toast";
import { walletIndexAtom } from "../jotai";
import { txFeedback } from "../utils/feedback";
import { useContract } from "./useContract";
import { contractCheck, updateEscrowQueries } from "../utils/helpers";
import { useWallet } from "../context/AppContext";
import { BigNumberish, bn } from "fuels";

interface UseReturnDepositProps {
    escrowId: BigNumberish;
}

export function useReturnDeposit({
    escrowId
}: UseReturnDepositProps) {
    const queryClient = useQueryClient();
    const wallet = useWallet();
    const walletIdx = useAtomValue(walletIndexAtom);
    const contract = useContract();
    const successMsg = "Deposit returned to buyer.";

    const mutation = useMutation(
        async () => {
            contractCheck(contract);

            const scope = await contract!.functions
                .return_deposit(escrowId)
                .txParams({
                    gasPrice: bn(5),
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
        queryClient.invalidateQueries(['EscrowPage-balances', walletIdx]);
        queryClient.invalidateQueries(["SellerEscrows", wallet?.address.toHexString()!]);
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