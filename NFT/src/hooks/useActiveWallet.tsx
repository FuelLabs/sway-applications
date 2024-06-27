import {
  useWallet,
  useIsConnected,
  useNetwork,
  useBalance,
} from "@fuels/react";

export const useActiveWallet = () => {
  const {
    wallet,
    isFetching: isWalletFetching,
    isLoading: isWalletLoading,
  } = useWallet();
  const { balance, refetch } = useBalance({
    address: wallet?.address.toB256(),
  });
  const {
    isConnected,
    isFetching: isConnectedFetching,
    isLoading: isConnectedLoading,
  } = useIsConnected();
  const { network } = useNetwork();

  return {
    wallet,
    walletBalance: balance,
    refetchBalnce: refetch,
    isPending:
      isWalletLoading ||
      isConnectedLoading ||
      isWalletFetching ||
      isConnectedFetching,
    isConnected,
    network,
  };
};
