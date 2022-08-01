import { toBigInt } from 'fuels';
import type { CoinQuantity } from 'fuels';

import { useBalances } from './useBalances';

import { CoinsMetadata } from '@/config';

const mergeCoinsWithMetadata = (coins: CoinQuantity[] = []) =>
  coins.map((coin) => {
    const coinMetadata = CoinsMetadata.find((c) => c.assetId === coin.assetId);
    return {
      name: coinMetadata?.name,
      assetId: coin.assetId,
      symbol: coinMetadata?.symbol,
      amount: toBigInt(coin.amount || 0),
    };
  });

export const useAssets = () => {
  const { data: balances } = useBalances();
  const coins = mergeCoinsWithMetadata(balances);
  return coins.sort((a, b) => {
    if (!a.symbol || !b.symbol) {
      return 0;
    }
    if (a.symbol < b.symbol) {
      return -1;
    }
    if (a.symbol > b.symbol) {
      return 1;
    }
    return 0;
  });
};
