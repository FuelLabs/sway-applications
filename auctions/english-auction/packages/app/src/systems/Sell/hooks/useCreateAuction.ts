import type { BigNumberish } from 'fuels';
import { useMutation } from 'react-query';

import { useContract } from '~/systems/Core/hooks/useContract';
import type {
  AuctionAssetInput,
  IdentityInput,
  OptionalU64Input,
} from '~/types/contracts/EnglishAuctionAbi';

interface UseCreateAuctionProps {
  bidAsset: AuctionAssetInput;
  duration: BigNumberish;
  initialPrice: BigNumberish;
  reservePrice: OptionalU64Input;
  seller: IdentityInput;
  sellAsset: AuctionAssetInput;
}

export function useCreateAuction({
  bidAsset,
  duration,
  initialPrice,
  reservePrice,
  seller,
  sellAsset,
}: UseCreateAuctionProps) {
  const contract = useContract();
  const mutation = useMutation(async () => {
    const callParams = sellAsset.TokenAsset ? sellAsset.TokenAsset : {};
    const result = await contract.functions
      .create(bidAsset, duration, initialPrice, reservePrice, seller, sellAsset)
      .callParams(callParams)
      .call();

    return result;
  });

  return mutation;
}
