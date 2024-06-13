import {
  useWallet,
  useIsConnected,
  useNetwork,
  useBalance,
} from "@fuels/react";

export const useActiveWallet = () => {
  const { wallet } = useWallet();
  const { balance, refetch } = useBalance({ address: wallet?.address.toB256() });
  const { isConnected } = useIsConnected();
  const { network } = useNetwork();

  return {
    wallet,
    walletBalance: balance,
    refetchBalnce: refetch,
    isConnected,
    network
  };
};
