import type { BN } from 'fuels';
import { useMutation } from 'react-query';

import { useContract } from '~/systems/Core/hooks/useContract';
import { handleError } from '~/systems/Core/utils';
import { txFeedback } from '~/systems/Core/utils/feedback';

interface UseWithdrawProps {
  auctionId: BN;
}

export const useWithdraw = ({ auctionId }: UseWithdrawProps) => {
  const { contract } = useContract();

  const mutation = useMutation(
    async () => {
      if (!contract) throw new Error('Contract not connected');

      const { transactionResult } = await contract.functions.withdraw(auctionId).call();
      return transactionResult;
    },
    {
      onSuccess: txFeedback('Withdraw from auction successful', handleSuccess),
      onError: handleError,
    }
  );

  function handleSuccess() {
    console.log('Withdraw success');
  }

  return mutation;
};
