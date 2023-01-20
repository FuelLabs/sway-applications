import { Address } from 'fuels';
import type { BigNumberish, CoinQuantityLike } from 'fuels';
import type { UseFormReturn } from 'react-hook-form';
import { useMutation } from 'react-query';

import type { CreateAuctionFormValues } from './useCreateAuctionForm';

import { useContract } from '~/systems/Core/hooks/useContract';
import { handleError } from '~/systems/Core/utils';
import { txFeedback } from '~/systems/Core/utils/feedback';
import type { AuctionAssetInput, IdentityInput } from '~/types/contracts/AuctionContractAbi';
import type { Option } from '~/types/contracts/common';

export type UseCreateAuctionProps = {
  bidAsset: AuctionAssetInput;
  duration: BigNumberish;
  initialPrice: BigNumberish;
  reservePrice: Option<BigNumberish>;
  sellerAddress: string;
  sellAsset: AuctionAssetInput;
};

export function useCreateAuction(form: UseFormReturn<CreateAuctionFormValues>) {
  const { contract } = useContract();
  const mutation = useMutation(
    async ({
      bidAsset,
      duration,
      initialPrice,
      reservePrice,
      sellerAddress,
      sellAsset,
    }: UseCreateAuctionProps) => {
      if (!contract) throw Error('Contract not connected');
      const callParams: CoinQuantityLike | undefined = sellAsset.TokenAsset ?? undefined;
      const seller: IdentityInput = {
        Address: { value: Address.fromString(sellerAddress).toHexString() },
      };

      const { transactionResult } = await contract.functions
        .create(bidAsset, duration, initialPrice, reservePrice, seller, sellAsset)
        .callParams({ forward: callParams })
        .call();
      return transactionResult;
    },
    {
      onSuccess: txFeedback('Auction created successfully!', handleSuccess),
      onError: handleError,
    }
  );

  function handleSuccess() {
    form.reset();
  }

  return mutation;
}
