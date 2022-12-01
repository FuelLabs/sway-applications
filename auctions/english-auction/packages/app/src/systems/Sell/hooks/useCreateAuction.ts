import { Address, CoinQuantityLike } from 'fuels';
import type { BigNumberish } from 'fuels';
import { useMutation } from 'react-query';

import { useContract } from '~/systems/Core/hooks/useContract';
import type {
  AuctionAssetInput,
  IdentityInput,
  OptionalU64Input,
} from '~/types/contracts/EnglishAuctionAbi';
import { txFeedback } from '~/systems/Core/utils/feedback';

import { handleError } from '~/systems/Core/utils';

export type UseCreateAuctionProps = {
  bidAsset: AuctionAssetInput;
  duration: BigNumberish;
  initialPrice: BigNumberish;
  reservePrice: OptionalU64Input;
  sellerAddress: string;
  sellAsset: AuctionAssetInput;
};

export function useCreateAuction({
  bidAsset,
  duration,
  initialPrice,
  reservePrice,
  sellerAddress,
  sellAsset,
}: UseCreateAuctionProps) {
  const contract = useContract();
  const mutation = useMutation(
    async () => {
      if (!contract) throw Error('Contract not connected');
      const callParams: CoinQuantityLike | undefined = sellAsset.TokenAsset ?? undefined;
      const seller: IdentityInput = {
        Address: { value: Address.fromString(sellerAddress).toHexString() },
      };

      console.log("bid asset: ", bidAsset);
      console.log("amount: ", bidAsset.TokenAsset?.amount.toString());
      console.log("duration: ", duration);
      console.log("inital price: ", initialPrice.toString());
      console.log("reserve price: ", reservePrice?.toString());
      console.log("seller address: ", seller);
      console.log("sell asset: ", sellAsset);
      console.log("amount: ", sellAsset.TokenAsset?.amount.toString());

      const { transactionResult } = await contract?.functions
        .create(bidAsset, duration, initialPrice, reservePrice, seller, sellAsset)
        .callParams({ forward: callParams })
        .call();
      return transactionResult;
    },
    {
      onSuccess: txFeedback("Auction created successfully!", handleSuccess),
      onError: handleError,
    }
  );

  function handleSuccess() {
    console.log("uwu");
  }

  return mutation;
}
