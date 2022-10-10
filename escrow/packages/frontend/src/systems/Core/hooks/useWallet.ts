import { useContext } from 'react';

import { AppContext } from '../context/AppContext';

export const useWallet = () => {
  const { wallet } = useContext(AppContext)!;
  return wallet;
};
