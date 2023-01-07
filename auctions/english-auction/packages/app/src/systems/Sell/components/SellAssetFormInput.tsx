import { Input, Stack } from "@fuel-ui/react";
import type { CoinQuantity } from "fuels";
import { useState } from "react";
import type { Control, FormState } from "react-hook-form";

import type { CreateAuctionFormValues } from "../hooks/useCreateAuctionForm";

import { DropdownContainerForm } from "./DropdownContainerForm";

import { ControlledField } from "~/systems/Core/components/ControlledField";

interface SellAuctionAssetFormInputProps {
  control: Control<CreateAuctionFormValues>;
  formState: FormState<CreateAuctionFormValues>;
  assets: CoinQuantity[];
}

export const SellAssetFormInput = ({
  control,
  formState,
  assets,
}: SellAuctionAssetFormInputProps) => {
  const [isNFT, setIsNFT] = useState(false);

  return (
    <DropdownContainerForm onChange={setIsNFT} assets={assets}>
      {!isNFT ? (
        <Stack css={{ minWidth: "%100" }}>
          <ControlledField
            control={control}
            name=""
            label=""
            isRequired
            isInvalid={Boolean(formState.errors.sellAssetAmount)}
            render={({ field }) => (
              <Input>
                <Input.Number
                  {...field}
                  aria-label="Sell asset amount"
                  placeholder="0.0"
                />
              </Input>
            )}
          />
        </Stack>
      ) : (
        <div>temp</div>
      )}
    </DropdownContainerForm>
  );
};
