import { Address } from 'fuels';
import type { BigNumberish } from 'fuels';
import { useMutation } from 'react-query';

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
  const mutation = useMutation(async () => {
    const callParams = sellAsset.TokenAsset ? sellAsset.TokenAsset : {};
    const seller: IdentityInput = {
      Address: { value: Address.fromString(sellerAddress).toHexString() },
    };
    const result = await contract.functions
      .create(bidAsset, duration, initialPrice, reservePrice, seller, sellAsset)
      .callParams(callParams)
      .call();

    return result;
  });

  return mutation;
}
