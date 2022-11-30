import { Address, CoinQuantityLike } from 'fuels';
import type { BigNumberish } from 'fuels';
import { useMutation } from 'react-query';
import toast from 'react-hot-toast';

import { useContract } from '~/systems/Core/hooks/useContract';
import type {
  AuctionAssetInput,
  IdentityInput,
  OptionalU64Input,
} from '~/types/contracts/EnglishAuctionAbi';

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
      console.log("bid", bidAsset.TokenAsset?.asset_id);
      console.log("sell", sellAsset.TokenAsset?.asset_id);
      const result = await contract?.functions
        .create(bidAsset, duration, initialPrice, reservePrice, seller, sellAsset)
        .callParams({ forward: callParams })
        .call();

      return result;
    }
  );

  return mutation;
}
