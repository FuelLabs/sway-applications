import { useQuery } from 'react-query';

import { useFuel } from './useFuel';

export const useLatestBlockHeight = () => {
  const [fuel] = useFuel();

  if (!fuel) {
    throw Error(`ERROR: fuel web3 is: ${fuel}`);
  }

  const { data: latestBlockHeight } = useQuery(['latestBlockHeight'], async () => {
    return await fuel.getProvider().getBlockNumber(); // eslint-disable-line @typescript-eslint/return-await
  });

  return latestBlockHeight;
};
