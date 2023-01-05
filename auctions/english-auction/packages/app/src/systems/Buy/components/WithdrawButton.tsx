import { Button } from "@fuel-ui/react";
import type { BN } from "fuels";

import { useDepositsBalance } from "../hooks/useDepositBalance";
import { useWithdraw } from "../hooks/useWithdraw";

import { useWallet } from "~/systems/Core/hooks/useWallet";

interface UseWithdrawButtonProps {
  auctionId: BN;
}

export const WithdrawButton = ({ auctionId }: UseWithdrawButtonProps) => {
  const withdrawMutation = useWithdraw({ auctionId });
  const wallet = useWallet();

  if (!wallet) throw new Error("Error wallet not connected");

  const balance = useDepositsBalance(auctionId, {
    Address: { value: wallet.address.toHexString() },
  });

  return (
    <>
      {balance && (
        <Button
          onPress={() => withdrawMutation.mutate()}
          css={{ minWidth: "100%" }}
        >
          Withdraw from Auction
        </Button>
      )}
    </>
  );
};
