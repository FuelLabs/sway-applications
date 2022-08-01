import { useAtomValue } from 'jotai';
import type { UseQueryOptions } from 'react-query';
import { useQuery } from 'react-query';

import { useWallet } from '../context/AppContext';
import { walletIndexAtom } from '../jotai';

export function useBalances(opts: UseQueryOptions = {}) {
  const walletIdx = useAtomValue(walletIndexAtom);
  const wallet = useWallet();
  return useQuery(['EscrowPage-balances', walletIdx], () => wallet?.getBalances(), opts as any); // eslint-disable-line @typescript-eslint/no-explicit-any
}
