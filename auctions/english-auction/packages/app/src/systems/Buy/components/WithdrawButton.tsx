import { Button } from "@fuel-ui/react";
import type { BN } from "fuels";

import { useDepositsBalance } from "../hooks/useDepositBalance";
import { useWithdraw } from "../hooks/useWithdraw";

import { useWallet } from "~/systems/Core/hooks/useWallet";
import type {
  AuctionAssetOutput,
  IdentityOutput,
} from "~/types/contracts/AuctionContractAbi";

interface UseWithdrawButtonProps {
  auctionId: BN;
  seller: IdentityOutput;
  bidAsset: AuctionAssetOutput;
  sellAsset: AuctionAssetOutput;
}

export const WithdrawButton = ({
  auctionId,
  seller,
  bidAsset,
  sellAsset,
}: UseWithdrawButtonProps) => {
  const { wallet } = useWallet();

  const balance = useDepositsBalance(auctionId, {
    Address: { value: wallet?.address.toHexString() || "" },
  });

  const withdrawMutation = useWithdraw({
    auctionId,
    seller,
    sellAsset,
    bidAsset,
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
