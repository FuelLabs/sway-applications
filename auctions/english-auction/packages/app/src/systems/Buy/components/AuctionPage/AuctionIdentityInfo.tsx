import { Flex } from "@fuel-ui/react";

import { AssetIdOutput, getSlicedAddress } from "~/systems/Core";
import type { IdentityOutput } from "~/types/contracts/AuctionContractAbi";
import type { Option } from "~/types/contracts/common";

interface AuctionIdentityInfoProps {
  sellerAddress: string;
  highestBidder: Option<IdentityOutput>;
}

export const AuctionIdentityInfo = ({
  sellerAddress,
  highestBidder,
}: AuctionIdentityInfoProps) => {
  return (
    <Flex>
      {/* TODO show bech32 address */}
      <AssetIdOutput
        assetId={getSlicedAddress(sellerAddress)}
        heading="Seller"
      />

      <AssetIdOutput
        assetId={
          (highestBidder && getSlicedAddress(highestBidder.Address!.value)) ||
          "None"
        }
        heading="Highest Bidder"
      />
    </Flex>
  );
};
