import { formatUnits } from 'ethers/lib/utils';
import type { BigNumberish } from 'fuels';

import type { EscrowAbi } from '../../../types/contracts';

export const formatValue = (amount: BigNumberish | null | undefined, decimals: number) => {
  if (amount != null) {
    return formatUnits(amount.toString(), decimals);
  }
  return '';
};

export const contractCheck = (contract: EscrowAbi | null | undefined) => {
  if (!contract) {
    throw new Error('Contract not found');
  }
};
