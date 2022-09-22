import { useAtomValue } from "jotai";
import toast from "react-hot-toast";
import { useMutation, useQueryClient } from "react-query";
import { walletIndexAtom } from "../jotai";
import { txFeedback } from "../utils/feedback";
import { parseInputValueBigInt } from "../utils/math";

import { useContract } from "./useContract";

interface UseDepositProps {
    depositAmount: string;
    depositAsset: string;
    escrowId: bigint;
}

export function useDeposit({
    depositAmount,
    depositAsset,
    escrowId,
}: UseDepositProps) {
    const queryClient = useQueryClient();
    const walletIdx = useAtomValue(walletIndexAtom);
    const contract = useContract();
    const successMsg = "Deposit successful.";

    const mutation = useMutation(
        async () => {
            if (!contract) {
                throw new Error("Contract not found");
            }

            const actualDeposit = parseInputValueBigInt(depositAmount)

            const scope = await contract!.functions
                .deposit(escrowId)
                .callParams({
                    forward: [actualDeposit, depositAsset]
                })
                .txParams({
                    gasPrice: BigInt(5),
                    bytePrice: BigInt(5),
                    gasLimit: 100_000_000,
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
        queryClient.fetchQuery(['EscrowPage-balances', walletIdx]);
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
                toast.error(`Error: ${errors[0].message}`)
            }
        } else {
            toast.error('Error when trying to deposit');
        }
    }

    return mutation;
}