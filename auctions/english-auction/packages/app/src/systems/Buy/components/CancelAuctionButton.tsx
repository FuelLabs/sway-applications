import { Button, Flex, Icon, Text } from "@fuel-ui/react";
import { bn } from "fuels";
import { useEffect, useState } from "react";

import { useCancelAuction } from "../hooks/useCancelAuctaion";

import { useWallet } from "~/systems/Core/hooks/useWallet";
import type { IdentityOutput } from "~/types/contracts/AuctionContractAbi";

interface UseCancelAuctionProps {
  index: number;
  seller: IdentityOutput;
}

export const CancelAuctionButton = ({
  index,
  seller,
}: UseCancelAuctionProps) => {
  const cancelAuctionMutation = useCancelAuction({ auctionId: bn(index) });
  const wallet = useWallet();
  const [identityOutput, setIdentityOutput] = useState<IdentityOutput>();

  if (!wallet) throw new Error("wallet not connected");

  useEffect(() => {
    const result: IdentityOutput = {
      Address: {
        value: wallet.address.toHexString()!,
      },
    };
    setIdentityOutput(result);
  }, [wallet]);

  return (
    <>
      {identityOutput?.Address?.value !== seller.Address?.value ? (
        <Text>
          <Flex gap="$4">
            <Icon icon="X" color="tomato10" />
            Error only the seller of the auction can cancel it.
          </Flex>
        </Text>
      ) : (
        <Button
          onPress={() => cancelAuctionMutation.mutate()}
          css={{ minWidth: "100%" }}
        >
          Cancel Auction
        </Button>
      )}
    </>
  );
};
