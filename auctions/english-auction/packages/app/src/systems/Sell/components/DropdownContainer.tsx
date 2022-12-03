import { Flex } from "@fuel-ui/react";
import type { CoinQuantity } from "fuels";

import { AuctionAssetDropdown } from "./AuctionAssetDropdown";

interface DropdownContainerProps {
  onChange: (isNFT: boolean, assetId: string) => void;
  children: JSX.Element;
  assets: CoinQuantity[];
}

export const DropdownContainer = ({
  children,
  assets,
  onChange,
}: DropdownContainerProps) => {
  return (
    <Flex>
      <Flex grow={2}>{children}</Flex>
      <Flex align="start" css={{ marginTop: "$9" }}>
        <AuctionAssetDropdown onChange={onChange} assets={assets} />
      </Flex>
    </Flex>
  );
};
