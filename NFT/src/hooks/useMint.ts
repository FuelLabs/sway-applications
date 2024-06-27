import { NFTContractAbi__factory } from "@/contract-types";
import { useWallet } from "@fuels/react";
import { useMutation } from "@tanstack/react-query";
import toast from "react-hot-toast";
import { useUpdateMetadata } from "./useUpdateMetadata";
import { CONTRACT_ID } from "@/lib";
import { queryClient } from "@/components/Provider";
import { NFTQueryKeys } from "@/queryKeys";

export const useMint = () => {
  const { wallet } = useWallet();
  const updateMetadata = useUpdateMetadata();

  const mutation = useMutation({
    mutationFn: async ({
      nftSubId,
    }: {
      cid: string;
      nftName: string;
      nftDescription: string;
      nftSubId: string;
    }) => {
      if (!wallet) throw new Error(`Cannot mint if wallet is ${wallet}`);

      const contract = NFTContractAbi__factory.connect(CONTRACT_ID, wallet);

      const recipient = { Address: { bits: wallet.address.toB256() } };

      const result = await contract.functions
        .mint(recipient, nftSubId, 1)
        .call();
      return result;
    },
    onSuccess: (_, { cid, nftName, nftDescription, nftSubId }) => {
      // Updating the info overwrites it so we also need to pass in past info
      updateMetadata.mutate({
        ipfsHash: cid,
        metadata: {
          keyvalues: {
            minter: wallet?.address.toB256() as string,
            nftName,
            nftDescription,
            nftSubId,
          },
        },
      });
      queryClient.invalidateQueries({ queryKey: [NFTQueryKeys.totalSupply] });
      toast.success("Successfully minted nft!");
    },
    onError: (err) => {
      // TODO: we need this in case the ts sdk
      // erroneously throws an error
      queryClient.invalidateQueries({ queryKey: [NFTQueryKeys.totalSupply] });
      console.error(err.message);
      toast.error(err.message);
    },
  });

  return mutation;
};
