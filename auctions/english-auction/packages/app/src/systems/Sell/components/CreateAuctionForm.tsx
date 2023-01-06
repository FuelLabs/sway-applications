import { Stack, Input } from "@fuel-ui/react";

import type { UseCreateAuctionFormReturn } from "../hooks/useCreateAuctionForm";

import { ControlledField } from "~/systems/Core/components/ControlledField";

export type CreateAuctionFormProps = {
  form: UseCreateAuctionFormReturn;
};

export const CreateAuctionForm = ({ form }: CreateAuctionFormProps) => {
  const { control, formState } = form;
  return (
    <Stack css={{ width: "%100" }} gap="$4">
      <ControlledField
        control={control}
        name="sellerAddress"
        label="SellerAddress"
        isRequired
        isInvalid={Boolean(formState.errors.sellerAddress)}
        render={({ field }) => (
          <Input>
            <Input.Field
              {...field}
              aria-label="Seller address"
              placeholder="0x000...000"
            />
          </Input>
        )}
      />
    </Stack>
  );
};
