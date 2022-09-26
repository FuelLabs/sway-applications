import type { BigNumberish } from 'fuels';
import { bn } from 'fuels';
import toast from 'react-hot-toast';
import { useMutation, useQueryClient } from 'react-query';

import { useWallet } from '../context/AppContext';
import { txFeedback } from '../utils/feedback';
import { contractCheck } from '../utils/helpers';

import { useContract } from './useContract';

interface UseDisputeProps {
  escrowId: BigNumberish;
}

export function useDispute({ escrowId }: UseDisputeProps) {
  const queryClient = useQueryClient();
  const wallet = useWallet();
  const contract = useContract();
  const successMsg = 'Dispute successful.';

  const mutation = useMutation(
    async () => {
      contractCheck(contract);

      const scope = await contract!.functions
        .dispute(escrowId)
        .txParams({
          gasPrice: bn(5),
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
    queryClient.invalidateQueries(['BuyerEscrows', wallet?.address.toHexString()]);
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
