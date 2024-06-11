import { NFTContractAbi__factory } from "@/contract-types";
import { useWallet } from "@fuels/react";
import { useMutation } from "@tanstack/react-query";
import toast from "react-hot-toast";
import { createSubId } from "@/utils/assetId";
import { hash } from "fuels";
import { useUpdateMetadata } from "./useUpdateMetadata";

export const useMint = () => {
  const { wallet } = useWallet();
  const updateMetadata = useUpdateMetadata();

  const mutation = useMutation({
    mutationFn: async ({
      totalAssets,
      contractId,
    }: {
      totalAssets: number;
      contractId: string;
      cid: string;
    }) => {
      if (!wallet) throw new Error(`Cannot mint if wallet is ${wallet}`);

      const contract = NFTContractAbi__factory.connect(contractId, wallet);

      const recipient = { Address: { bits: wallet.address.toB256() } };
      // We need to hash subId
      const subId = hash(`0x${createSubId(totalAssets + 1)}`);

      const result = await contract.functions.mint(recipient, subId, 1).call();
      return result;
    },
    onSuccess: (_, { cid }) => {
      updateMetadata.mutate({
        ipfsHash: cid,
        metadata: { keyvalues: { minter: wallet?.address.toB256() as string } },
      });
      toast.success("Successfully minted nft!");
    },
    onError: (err) => {
      toast.error(err.message);
    },
  });

  return mutation;
};
