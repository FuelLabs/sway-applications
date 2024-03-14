import { useMutation } from "@tanstack/react-query";

import { useWallet } from "@fuels/react";
import { TictactoeContractAbi__factory } from "../contract-types";
import { queryClient } from "../components";
import { CONTRACT_ID } from "../config";

export const useMakeMove = (position: number) => {
  const { wallet } = useWallet();

  const mutation = useMutation({
    mutationFn: async () => {
      if (!wallet) throw new Error(`Cannot make move if wallet is ${wallet}`);

      const contract = TictactoeContractAbi__factory.connect(
        CONTRACT_ID,
        wallet
      );
      const result = await contract.functions.make_move(position).call();
      return result;
    },
    onSuccess: async () => {
      await queryClient.invalidateQueries();
    },
    onError: (err) => {
      console.error(err);
    },
  });

  return mutation;
};
