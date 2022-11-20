import { useQuery } from 'react-query';

export const useWallet = () => {
  const { data: wallet } = useQuery(['wallet'], async () => {
    const accounts = await window.FuelWeb3.accounts();
    // TODO don't hardcode accounts[0]
    return window.FuelWeb3.getWallet(accounts[0]);
  });

  return wallet;
};
