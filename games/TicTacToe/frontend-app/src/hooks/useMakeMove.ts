import { useWallet } from '@fuels/react';
import { useMutation } from '@tanstack/react-query';

import { queryClient } from '../components';
import { CONTRACT_ID } from '../config';
import { TictactoeContractAbi__factory } from '../contract-types';

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
    onError: async (err) => {
      // TODO: remove once we figure out why a successful call returns an error from the ts sdk
      await queryClient.invalidateQueries();
      // eslint-disable-next-line no-console
      console.error(err);
    },
  });

  return mutation;
};
