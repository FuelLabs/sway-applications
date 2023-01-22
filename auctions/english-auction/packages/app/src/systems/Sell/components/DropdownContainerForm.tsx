import { Flex } from "@fuel-ui/react";
import type { CoinQuantity } from "fuels";
import { Controller } from "react-hook-form";
import type { Control, UseFormSetValue } from "react-hook-form";

import type { CreateAuctionFormValues } from "../hooks/useCreateAuctionForm";

import { AuctionAssetDropdown } from "./AuctionAssetDropdown";

interface DropdownContainerProps {
  children: JSX.Element;
  assets: CoinQuantity[];
  formFieldName: "isSellAssetNft" | "isBidAssetNft";
  control: Control<CreateAuctionFormValues>;
  setValue: UseFormSetValue<CreateAuctionFormValues>;
  setValueLabel: "sellAssetId" | "bidAssetId";
  ariaLabel: string;
}

export const DropdownContainerForm = ({
  children,
  assets,
  formFieldName,
  control,
  setValue,
  setValueLabel,
  ariaLabel,
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
            function handleDropdownChange(isNFT: boolean, assetId: string) {
              field.onChange(isNFT);
              setValue(setValueLabel, assetId);
            }

            return (
              <AuctionAssetDropdown
                onChange={handleDropdownChange}
                assets={assets}
                ariaLabel={ariaLabel}
              />
            );
          }}
        />
      </Flex>
    </Flex>
  );
};
