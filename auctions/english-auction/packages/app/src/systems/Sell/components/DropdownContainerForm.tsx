import { Flex } from "@fuel-ui/react";
import type { CoinQuantity } from "fuels";
import { Controller } from "react-hook-form";
import type { Control } from "react-hook-form";

import type { CreateAuctionFormValues } from "../hooks/useCreateAuctionForm";

import { AuctionAssetDropdown } from "./AuctionAssetDropdown";

interface DropdownContainerProps {
  children: JSX.Element;
  assets: CoinQuantity[];
  formFieldName: "isSellAssetNft" | "isBidAssetNft";
  control: Control<CreateAuctionFormValues>;
}

export const DropdownContainerForm = ({
  children,
  assets,
  formFieldName,
  control,
}: DropdownContainerProps) => {
  return (
    <Flex>
      <Flex grow={2}>{children}</Flex>
      <Flex
        align="start"
        css={{ marginTop: "$9", marginLeft: "$2", marginRight: "$2" }}
      >
        <Controller
          name={formFieldName}
          control={control}
          render={({ field }) => {
            return (
              <AuctionAssetDropdown onChange={field.onChange} assets={assets} />
            );
          }}
        />
      </Flex>
    </Flex>
  );
};
