import { NFTContractAbi__factory } from "@/contract-types";
import { CONTRACT_ID } from "@/lib";
import { useWallet } from "@fuels/react";
import { useMutation } from "@tanstack/react-query";
import { getRandomB256 } from "fuels";
import toast from "react-hot-toast";

export const useMint = () => {
  const { wallet } = useWallet();

  const mutation = useMutation({
    mutationFn: async () => {
      if (!wallet) throw new Error(`Cannot mint if wallet is ${wallet}`);

      const contract = NFTContractAbi__factory.connect(CONTRACT_ID, wallet);

      const recipient = { Address: { bits: wallet.address.toB256() } }
      // TODO: is this the best way to generate subId?
      const subId = getRandomB256();
      const result = await contract.functions.mint(recipient, subId, 1).call();
      return result;
    },
    onError: (err) => {
      toast.error(err.message);
    },
  });

  return mutation;
};
