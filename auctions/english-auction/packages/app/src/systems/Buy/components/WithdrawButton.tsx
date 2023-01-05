import { Button } from "@fuel-ui/react";
import type { BN } from "fuels";

import { useWithdraw } from "../hooks/useWithdraw";

interface UseWithdrawButtonProps {
  auctionId: BN;
}

export const WithdrawButton = ({ auctionId }: UseWithdrawButtonProps) => {
  const withdrawMutation = useWithdraw({ auctionId });

  return (
    <Button
      onPress={() => withdrawMutation.mutate()}
      css={{ minWidth: "100%" }}
    >
      Withdraw from Auction
    </Button>
  );
};
