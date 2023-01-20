import type { BN, CoinQuantityLike } from 'fuels';
import { useMutation } from 'react-query';

import { useContract } from '~/systems/Core/hooks/useContract';
import { handleError, queryClient } from '~/systems/Core/utils';
import { txFeedback } from '~/systems/Core/utils/feedback';
import type { AuctionAssetInput } from '~/types/contracts/AuctionContractAbi';

interface UseBidProps {
  auctionId: BN;
  auctionAsset: AuctionAssetInput;
  setAssetAmount: React.Dispatch<React.SetStateAction<string>>;
}

export const useBid = ({ auctionId, auctionAsset, setAssetAmount }: UseBidProps) => {
  const { contract } = useContract();

  const mutation = useMutation(
    async () => {
      if (!contract) throw new Error('Contract not connected');
      const callParams: CoinQuantityLike | undefined = auctionAsset.TokenAsset ?? undefined;

      const { transactionResult } = await contract.functions
        .bid(auctionId, auctionAsset)
        .callParams({ forward: callParams })
        .call();

      return transactionResult;
    },
    {
      onSuccess: txFeedback('Auction bid placed successfully', handleSuccess),
      onError: handleError,
    }
  );

  function handleSuccess() {
    queryClient.invalidateQueries({ queryKey: ['auctionInfo'] });
    queryClient.invalidateQueries({ queryKey: ['totalAuctions'] });
    setAssetAmount('');
  }

  return mutation;
};
