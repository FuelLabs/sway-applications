import { useWallet } from "@fuels/react";
import { useMutation, useQueryClient } from "@tanstack/react-query";
import toast from "react-hot-toast";
import { getRandomB256 } from "fuels";
import { NFTContractAbi__factory } from "@/contract-types";
import { AssetIdInput } from "@/contract-types/contracts/NFTContractAbi";
import { createAssetId } from "@/utils/assetId";
import { useUpdateMetadata } from "./useUpdateMetadata";
import { useUnpin } from "./useUnpin";
import { CONTRACT_ID } from "@/lib";
import { useRouter } from "next/router";
import { NFTQueryKeys } from "@/queryKeys";

type CreateNFT = {
  cid: string;
  name: string;
  symbol: string;
  description: string;
};

export const useCreateNFT = () => {
  const { wallet } = useWallet();
  const updateMetadata = useUpdateMetadata();
  const unpin = useUnpin();
  const router = useRouter();
  const queryClient = useQueryClient();

  const mutation = useMutation({
    mutationKey: [NFTQueryKeys.createNFT],
    mutationFn: async ({ cid, name, symbol }: CreateNFT) => {
      if (!wallet)
        throw new Error(
          `Cannot create NFT if wallet is ${wallet}.  Please connect your wallet.`
        );

      const contract = NFTContractAbi__factory.connect(CONTRACT_ID, wallet);

      let contractCalls = [];
      const subId = getRandomB256();
      const assetId: AssetIdInput = createAssetId(subId, CONTRACT_ID);
      contractCalls.push(
        contract.functions.set_metadata(assetId, "image", { String: cid })
      );
      contractCalls.push(contract.functions.set_name(assetId, name));
      contractCalls.push(contract.functions.set_symbol(assetId, symbol));
      await contract.multiCall(contractCalls).call();
      return subId;
    },
    onSuccess: (data, { cid, name, description }) => {
      updateMetadata.mutateAsync({
        ipfsHash: cid,
        metadata: {
          keyvalues: {
            nftSubId: data,
            nftName: name,
            nftDescription: description,
          },
        },
      });
      toast.success("NFT successfully created.");
      router.push("/nft");
    },
    onError: (err, { cid }) => {
      // TODO: if the ts sdk erroneously throws an error
      // this will delete the ipfs image
      unpin.mutate({ ipfsHash: cid });
      queryClient.invalidateQueries({ queryKey: [NFTQueryKeys.nftData] });
      console.error(err.message);
      toast.error(err.message);
    },
  });

  return mutation;
};
