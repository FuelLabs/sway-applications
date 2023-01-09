import { Input, Stack } from "@fuel-ui/react";
import type { CoinQuantity } from "fuels";
import { useEffect } from "react";

import type { UseCreateAuctionFormReturn } from "../../hooks/useCreateAuctionForm";
import { AddressFormInput } from "../AddressFormInput";
import { BidAassetFormInput } from "../BidAssetFormInput";
import { ReservePriceInput } from "../ReservePriceInput";
import { SellAssetFormInput } from "../SellAssetFormInput";

import { ControlledField } from "~/systems/Core/components/ControlledField";

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
  const { control, formState, watch, setValue } = form;
  const watchHasReservePrice = watch("hasReservePrice", false);
  const watchIsSellAssetNft = watch("isSellAssetNft", false);
  const watchIsBidAssetNft = watch("isBidAssetNft", false);

  useEffect(() => {
    if (assets.length !== 0) {
      setValue("sellAssetId", assets[0].assetId);
      setValue("bidAssetId", assets[0].assetId);
    }
  }, [assets]);

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
        isSellAssetNft={watchIsSellAssetNft}
        setValue={setValue}
      />

      <ControlledField
        control={control}
        name="initialPrice"
        label="Initial Price"
        isRequired
        isInvalid={Boolean(formState.errors.initialPrice)}
        render={({ field }) => (
          <Input>
            <Input.Number
              {...field}
              aria-label="Initial price"
              placeholder="0.0"
              allowNegative={false}
            />
          </Input>
        )}
      />

      <ReservePriceInput
        control={control}
        formState={formState}
        hasReservePrice={watchHasReservePrice}
      />

      <BidAassetFormInput
        assets={assets}
        control={control}
        formState={formState}
        isBidAssetNft={watchIsBidAssetNft}
      />

      <ControlledField
        control={control}
        name="duration"
        label="Duration"
        isRequired
        isInvalid={Boolean(formState.errors.duration)}
        render={({ field }) => (
          <Input>
            <Input.Number
              {...field}
              arial-label="Duration"
              placeholder="0"
              allowNegative={false}
            />
          </Input>
        )}
      />
    </Stack>
  );
};
