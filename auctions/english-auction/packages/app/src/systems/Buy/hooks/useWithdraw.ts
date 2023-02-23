import { Contract } from 'fuels';
import type { BN, BigNumberish } from 'fuels';
import { useMutation } from 'react-query';

import { useContract } from '~/systems/Core/hooks/useContract';
import { handleError, queryClient } from '~/systems/Core/utils';
import { txFeedback } from '~/systems/Core/utils/feedback';
import { NFTAbi__factory } from '~/types/contracts';
import type {
  AuctionContractAbi,
  IdentityInput,
  IdentityOutput,
  AuctionAssetOutput,
} from '~/types/contracts/AuctionContractAbi';

interface UseWithdrawProps {
  auctionId: BN;
  seller: IdentityOutput;
  bidAsset: AuctionAssetOutput;
  sellAsset: AuctionAssetOutput;
}

async function withdrawAsset(
  asset: AuctionAssetOutput,
  contract: AuctionContractAbi,
  auctionId: BigNumberish
) {
  if (asset.NFTAsset) {
    const res = await contract.functions
      .withdraw(auctionId)
      .addContracts([
        new Contract(
          asset.NFTAsset.asset_id.value,
          NFTAbi__factory.createInterface(),
          contract.provider!
        ),
      ])
      .call();
    return res;
  }
  const res = await contract.functions
    .withdraw(auctionId)
    .addContracts([
      new Contract(process.env.VITE_NFT_ID!, NFTAbi__factory.createInterface(), contract.provider!),
    ])
    .call();
  return res;
}

export const useWithdraw = ({ auctionId, seller, bidAsset, sellAsset }: UseWithdrawProps) => {
  const { contract } = useContract();

  const mutation = useMutation(
    async () => {
      if (!contract) throw new Error('Contract not connected');

      const sellerAddress = seller.Address ? seller.Address.value! : seller.ContractId!.value!;
      // Sellers
      const { transactionResult } =
        sellerAddress === contract.wallet?.address.toHexString()
          ? await withdrawAsset(bidAsset, contract, auctionId)
          : await withdrawAsset(sellAsset, contract, auctionId);
      return transactionResult;
    },
    {
      onSuccess: txFeedback('Withdraw from auction successful', handleSuccess),
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
