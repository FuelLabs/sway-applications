import { Input, Stack } from "@fuel-ui/react";
import type { CoinQuantity } from "fuels";
import type { Control, FormState } from "react-hook-form";

import type { CreateAuctionFormValues } from "../hooks/useCreateAuctionForm";

import { DropdownContainerForm } from "./DropdownContainerForm";

import { ControlledField } from "~/systems/Core/components/ControlledField";

interface SellAuctionAssetFormInputProps {
  control: Control<CreateAuctionFormValues>;
  formState: FormState<CreateAuctionFormValues>;
  assets: CoinQuantity[];
  isSellAssetNft: boolean;
}

export const SellAssetFormInput = ({
  control,
  formState,
  assets,
  isSellAssetNft,
}: SellAuctionAssetFormInputProps) => {
  // const [isNFT, setIsNFT] = useState(false);

  return (
    <DropdownContainerForm
      assets={assets}
      formFieldName="isSellAssetNft"
      control={control}
    >
      {!isSellAssetNft ? (
        <Stack css={{ minWidth: "100%" }}>
          <ControlledField
            control={control}
            name="sellAssetAmount"
            label="Sell Asset Amount"
            isRequired
            isInvalid={Boolean(formState.errors.sellAssetAmount)}
            render={({ field }) => (
              <Input css={{ minWidth: "100%" }}>
                <Input.Number
                  {...field}
                  aria-label="Sell asset amount"
                  placeholder="0.0"
                  allowNegative={false}
                />
              </Input>
            )}
          />
        </Stack>
      ) : (
        <Stack css={{ minWidth: "100%", marginRight: "$2" }}>
          <ControlledField
            control={control}
            name="sellNFTTokenId"
            label="Sell NFT Id"
            isRequired
            isInvalid={Boolean(formState.errors.sellNFTTokenId)}
            render={({ field }) => (
              <Input>
                <Input.Number
                  {...field}
                  aria-label="Sell nft token id"
                  placeholder="0"
                  allowNegative={false}
                  autoComplete="off"
                />
              </Input>
            )}
          />
          <ControlledField
            control={control}
            name="sellNFTAssetId"
            label="Sell NFT Asset Id"
            isRequired
            isInvalid={Boolean(formState.errors.sellNFTAssetId)}
            render={({ field }) => (
              <Input>
                <Input.Field
                  {...field}
                  aria-label="Sell nft asset id"
                  placeholder="0x000...000"
                />
              </Input>
            )}
          />
        </Stack>
      )}
    </DropdownContainerForm>
  );
};
