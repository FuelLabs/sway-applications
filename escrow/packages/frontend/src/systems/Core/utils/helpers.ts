import { formatUnits } from "ethers/lib/utils";
import { QueryClient, useQueryClient } from "react-query";
import { useWallet } from "../context/AppContext";

import type { EscrowAbi } from "@/types/contracts";
import { Maybe } from "@/types";
import { BigNumberish, Wallet } from "fuels";

export   const formatValue = (amount: BigNumberish | null | undefined, decimals: number) => {
  if (amount != null) {
    return formatUnits(amount.toString(), decimals);
  }
  return "";
};

export const updateEscrowQueries = (queryClient: QueryClient, wallet: Maybe<Wallet>) => {
  queryClient.invalidateQueries(["SellerPage-sellerEscrowIds", wallet]);
  queryClient.invalidateQueries(["BuyerPage-buyerEscrowIds", wallet]);
  queryClient.invalidateQueries(["ArbiterPage-arbiterEscrowIds", wallet]);
}

export const contractCheck = (contract: EscrowAbi | null | undefined ) => {
  if (!contract) {
    throw new Error('Contract not found');
  }
}