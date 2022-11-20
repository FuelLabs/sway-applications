export const useConnectWallet = async () => {
  await window.FuelWeb3.connect();
};
