import { Stack } from "@fuel-ui/react";

import type { UseCreateAuctionFormReturn } from "../../hooks/useCreateAuctionForm";
import { AddressFormInput } from "../AddressFormInput";

export type CreateAuctionFormProps = {
  form: UseCreateAuctionFormReturn;
  walletAddress: string;
};

export const CreateAuctionForm = ({
  form,
  walletAddress,
}: CreateAuctionFormProps) => {
  const { control, formState } = form;

  return (
    <Stack css={{ width: "%100" }} gap="$4">
      <AddressFormInput
        control={control}
        formState={formState}
        walletAddress={walletAddress}
      />
    </Stack>
  );
};
