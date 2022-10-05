import type { BigNumberish } from 'fuels';
import { bn } from 'fuels';
import { useAtomValue } from 'jotai';
import toast from 'react-hot-toast';
import { useMutation, useQueryClient } from 'react-query';

import type { ArbiterInput } from '../../../types/contracts/EscrowAbi';
import { walletIndexAtom } from '../jotai';
import { txFeedback } from '../utils/feedback';
import { contractCheck } from '../utils/helpers';
import { parseInputValueBigInt } from '../utils/math';

import { useContract } from './useContract';

interface UseProposeArbiterProps {
  arbiterFee: string;
  arbiterAddress: string;
  arbiterAsset: string;
  escrowId: BigNumberish;
  setArbiterFee: React.Dispatch<React.SetStateAction<string>>;
  setArbiterAddress: React.Dispatch<React.SetStateAction<string>>;
  setArbiterAsset: React.Dispatch<React.SetStateAction<string>>;
}

export function useProposeArbiter({
  arbiterAddress,
  arbiterAsset,
  arbiterFee,
  escrowId,
  setArbiterAddress,
  setArbiterAsset,
  setArbiterFee,
}: UseProposeArbiterProps) {
  const queryClient = useQueryClient();
  const walletIdx = useAtomValue(walletIndexAtom);
  const contract = useContract();
  const successMsg = 'New arbiter proposed.';

  const mutation = useMutation(
    async () => {
      contractCheck(contract);

      const actualFee = parseInputValueBigInt(arbiterFee);
      const arbiterArg: ArbiterInput = {
        address: { Address: { value: arbiterAddress } },
        asset: { value: arbiterAsset },
        fee_amount: actualFee,
      };

      const scope = await contract!.functions
        .propose_arbiter(arbiterArg, escrowId)
        .callParams({
          forward: [actualFee, arbiterAsset],
        })
        .txParams({
          gasPrice: bn(5),
          gasLimit: 100_000_000,
          variableOutputs: 1,
        })
        .fundWithRequiredCoins();

      const response = await contract!.wallet!.sendTransaction(scope.transactionRequest);
      const result = await response.waitForResult();

      return result;
    },
    {
      onSuccess: txFeedback(successMsg, handleSuccess),
      onError: handleError,
    }
  );

  function handleSuccess() {
    setArbiterAddress('');
    setArbiterAsset('');
    setArbiterFee('');

    queryClient.invalidateQueries(['EscrowPage-balances', walletIdx]);
  }

  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  function handleError(e: any) {
    const errors = e?.response?.errors;

    if (errors?.length) {
      if (errors[0].message === 'enough coins could not be found') {
        toast.error(`Not enough balance in your wallet to create an escrow`);
      }
    } else {
      toast.error(`Error when trying to propose arbiter`);
    }
  }

  return mutation;
}
