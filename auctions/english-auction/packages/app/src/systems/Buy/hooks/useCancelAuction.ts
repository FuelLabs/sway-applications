import type { BN } from 'fuels';
import { useMutation } from 'react-query';

import { useContract } from '~/systems/Core/hooks/useContract';
import { handleError, queryClient } from '~/systems/Core/utils';
import { txFeedback } from '~/systems/Core/utils/feedback';
import type { IdentityInput } from '~/types/contracts/AuctionContractAbi';

interface UseCancelAuctionProps {
  auctionId: BN;
}

export const useCancelAuction = ({ auctionId }: UseCancelAuctionProps) => {
  const { contract } = useContract();

  const mutation = useMutation(
    async () => {
      if (!contract) throw Error('Contract not connected');

      const { transactionResult } = await contract.functions.cancel(auctionId).call();
      return transactionResult;
    },
    {
      onSuccess: txFeedback('Auction cancelled successfully!', handleSuccess),
      onError: handleError,
    }
  );

  function handleSuccess() {
    queryClient.invalidateQueries({ queryKey: ['totalAuctions'] });
    queryClient.invalidateQueries({ queryKey: ['auctionInfo'] });
    const temp: IdentityInput = { Address: { value: contract!.wallet!.address!.toString()! } };
    queryClient.invalidateQueries({ queryKey: ['depositBalance', auctionId.toString(), temp] });
  }

  return mutation;
};
