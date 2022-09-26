import { IdentityInput } from "@/types/contracts/EscrowAbi";
import { useAtomValue } from "jotai";
import { useMutation, useQueryClient } from "react-query";
import toast from 'react-hot-toast';
import { walletIndexAtom } from "../jotai";
import { txFeedback } from "../utils/feedback";
import { parseInputValueBigInt } from "../utils/math";
import { useContract } from "./useContract";
import { updateEscrowQueries } from "../utils/helpers";
import { useWallet } from "../context/AppContext";
import { BigNumberish, bn } from "fuels";

interface UseResolveDisputeProps {
    escrowId: BigNumberish,
    arbiterPayment: string,
    favoredUser: string,
}

export function useResolveDispute({
    escrowId,
    arbiterPayment,
    favoredUser,
}: UseResolveDisputeProps) {
    const queryClient = useQueryClient();
    const wallet = useWallet();
    const walletIdx = useAtomValue(walletIndexAtom);
    const contract = useContract();
    const successMsg = "Dispute resolved.";

    const mutation = useMutation(
       async () => {
            if (!contract) {
                throw new Error('Contract not found');
            }

            const actualPayment = parseInputValueBigInt(arbiterPayment)
            const userArg: IdentityInput = { Address: { value: favoredUser } };
            const scope = await contract!.functions
                .resolve_dispute(escrowId, actualPayment, userArg)
                .txParams({
                    gasPrice: bn(5),
                    gasLimit: 100_000_000,
                    variableOutputs: 3
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
        // Trigger query to update blanaces etc
        queryClient.invalidateQueries(['EscrowPage-balances', walletIdx]);
        queryClient.invalidateQueries(["ArbiterEscrows", wallet?.address.toHexString()!]);
    }

    function handleError(e: any) {
        const errors = e?.response?.errors;

        if (errors?.length) {
            if (errors[0].message === 'enough coins could not be found') {
                toast.error(
                    `Not enough balance in your wallet to create an escrow`
                );
            }
        } else {
            toast.error(`Error when trying to create an escrow`);
        }
    }

    return mutation;
}