import { formatUnits } from "ethers/lib/utils";
import { useQueryClient } from "react-query";
import { useWallet } from "../context/AppContext";

export   const formatValue = (amount: bigint | null | undefined, decimals: number) => {
  if (amount != null) {
    return formatUnits(amount, decimals);
  }
  return "";
};

export const updateEscrowQueries = () => {
  const queryClient = useQueryClient();
  const wallet = useWallet();
  queryClient.invalidateQueries(["SellerEscrows", wallet]);
  queryClient.invalidateQueries(["BuyerEscrows", wallet]);
  queryClient.invalidateQueries(["ArbiterEscrows", wallet]);
}