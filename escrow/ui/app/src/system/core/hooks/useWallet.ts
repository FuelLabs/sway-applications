import { useQuery } from "@tanstack/react-query";
import { useFuel } from "./useFuel";

export const useWallet = () => {
  const fuel = useFuel();

  const { data: isConnected } = useQuery(
    ["isConnected"],
    async () => {
      return await fuel!.isConnected();
    },
    {
      enabled: !!fuel,
    }
  );

  const {
    data: wallet,
    isLoading,
    isError,
  } = useQuery(
    ["wallet"],
    async () => {
      const selectedAccount = (await fuel!.currentAccount()) as string;
      const selectedWallet = await fuel!.getWallet(selectedAccount);
      return selectedWallet;
    },
    {
      enabled: !!fuel && isConnected,
    }
  );

  return { wallet, isLoading, isError };
};
