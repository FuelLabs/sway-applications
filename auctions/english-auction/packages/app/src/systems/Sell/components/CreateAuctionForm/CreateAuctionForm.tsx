import { Stack } from "@fuel-ui/react";
import type { CoinQuantity } from "fuels";

import type { UseCreateAuctionFormReturn } from "../../hooks/useCreateAuctionForm";
import { AddressFormInput } from "../AddressFormInput";
import { SellAssetFormInput } from "../SellAssetFormInput";

export type CreateAuctionFormProps = {
  form: UseCreateAuctionFormReturn;
  walletAddress: string;
  assets: CoinQuantity[];
};

export const CreateAuctionForm = ({
  form,
  walletAddress,
  assets,
}: CreateAuctionFormProps) => {
  const { control, formState } = form;

  return (
    <Stack css={{ width: "%100" }} gap="$4">
      <AddressFormInput
        control={control}
        formState={formState}
        walletAddress={walletAddress}
      />

      <SellAssetFormInput
        control={control}
        formState={formState}
        assets={assets}
      />
    </Stack>
  );
};
