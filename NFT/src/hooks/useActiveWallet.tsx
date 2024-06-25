import {
  useWallet,
  useIsConnected,
  useNetwork,
  useBalance,
} from "@fuels/react";

export const useActiveWallet = () => {
  const { wallet, isPending: isWalletPending } = useWallet();
  const {
    balance,
    refetch,
  } = useBalance({ address: wallet?.address.toB256() });
  const { isConnected, isPending: isConnectedPending } = useIsConnected();
  const { network } = useNetwork();

  return {
    wallet,
    walletBalance: balance,
    refetchBalnce: refetch,
    isPending:
      isWalletPending ||
      isConnectedPending,
    isConnected,
    network,
  };
};
