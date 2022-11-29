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
  const mutation = useMutation(async () => {
    if (!contract) throw Error("Contract not connected");
    const callParams: CoinQuantityLike | undefined = sellAsset.TokenAsset ?? undefined;
    const seller: IdentityInput = {
      Address: { value: Address.fromString(sellerAddress).toHexString() },
    };
    console.log("contract wallet: ", contract.wallet);
    const result = await contract?.functions
      .create(bidAsset, duration, initialPrice, reservePrice, seller, sellAsset)
      .callParams({ forward: callParams })
      .call();

    console.log("result: ", result);

    return result;
  },
  {
    onError: handleError,
  });

    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    function handleError(e: any) {
      const errors = e?.response?.errors;

      if (errors?.length) {
        if (errors[0].message === 'enough coins could not be found') {
          toast.error(`Not enough balance in your wallet to create an escrow`);
        }
      } else {
        toast.error(errors[0].message);
      }
    }

  return mutation;
}
