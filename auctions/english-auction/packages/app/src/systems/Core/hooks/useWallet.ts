export const useWallet = async () => {
  await window.FuelWeb3.connect();
};
