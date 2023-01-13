import type { BN, CoinQuantityLike } from 'fuels';
import { useMutation } from 'react-query';

import { useContract } from '~/systems/Core/hooks/useContract';
import { handleError } from '~/systems/Core/utils';
import { txFeedback } from '~/systems/Core/utils/feedback';
import type { AuctionAssetInput } from '~/types/contracts/EnglishAuctionAbi';

interface UseBidProps {
  auctionId: BN;
  auctionAsset: AuctionAssetInput;
}

export const useBid = ({ auctionId, auctionAsset }: UseBidProps) => {
  const { contract } = useContract();

  const mutation = useMutation(
    async () => {
      console.log('ASDFASDFASDFASDF');
      if (!contract) throw new Error('Contract not connected');
      console.log('cur address', contract.wallet?.address.toString());
      const callParams: CoinQuantityLike | undefined = auctionAsset.TokenAsset ?? undefined;

      console.log('call params: ', callParams);
      console.log('id: ', auctionId);
      console.log('auction asset', auctionAsset);
      const { transactionResult } = await contract.functions
        .bid(auctionId, auctionAsset)
        .callParams({ forward: callParams })
        .call();

      console.log('res', transactionResult);

      return transactionResult;
    },
    {
      onSuccess: txFeedback('Auction bid placed successfully', handleSuccess),
      onError: handleError,
    }
  );

  function handleSuccess() {
    console.log('bid success');
  }

  return mutation;
};
