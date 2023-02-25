import { useQuery } from 'react-query';

import { useWallet } from './useWallet';

export const useLatestBlockHeight = () => {
  const { wallet, isLoading, isError } = useWallet();

  const {
    data: latestBlockHeight,
    isLoading: isBlockHeightLoading,
    isError: isBlockHeightError,
  } = useQuery(
    ['latestBlockHeight'],
    async () => {
      return await wallet!.provider.getBlockNumber(); // eslint-disable-line @typescript-eslint/return-await
    },
    {
      enabled: !isLoading && !isError && !!wallet,
    }
  );

  return { latestBlockHeight, isLoading: isBlockHeightLoading, isError: isBlockHeightError };
};
