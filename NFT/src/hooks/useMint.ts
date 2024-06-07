import { NFTContractAbi__factory } from "@/contract-types";
import { useWallet } from "@fuels/react";
import { useMutation } from "@tanstack/react-query";
import toast from "react-hot-toast";
import { createSubId } from "@/utils/assetId";

export const useMint = () => {
  const { wallet } = useWallet();

  const mutation = useMutation({
    mutationFn: async ({
      totalAssets,
      contractId,
    }: {
      totalAssets: number;
      contractId: string;
    }) => {
      if (!wallet) throw new Error(`Cannot mint if wallet is ${wallet}`);

      const contract = NFTContractAbi__factory.connect(contractId, wallet);

      const recipient = { Address: { bits: wallet.address.toB256() } };
      const subId = createSubId(totalAssets + 1);
      const result = await contract.functions.mint(recipient, subId, 1).call();
      return result;
    },
    onSuccess: () => {
      toast.success("Successfully minted nft!");
    },
    onError: (err) => {
      toast.error(err.message);
    },
  });

  return mutation;
};
