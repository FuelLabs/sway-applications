import { Form, Input, Stack } from "@fuel-ui/react";
import type { CoinQuantity } from "fuels";
import type { Control, FormState, UseFormSetValue } from "react-hook-form";

import type { CreateAuctionFormValues } from "../hooks/useCreateAuctionForm";

import { DropdownContainerForm } from "./DropdownContainerForm";

import { ControlledField } from "~/systems/Core/components/ControlledField";

interface BidAassetFormInputProps {
  assets: CoinQuantity[];
  control: Control<CreateAuctionFormValues>;
  formState: FormState<CreateAuctionFormValues>;
  isBidAssetNft: boolean;
  setValue: UseFormSetValue<CreateAuctionFormValues>;
}

export const BidAassetFormInput = ({
  assets,
  control,
  formState,
  isBidAssetNft,
  setValue,
}: BidAassetFormInputProps) => {
  return (
    <Stack css={{ minWidth: "100%" }}>
      <Form.Control>
        <Form.Label>Bid Asset</Form.Label>
      </Form.Control>
      <DropdownContainerForm
        assets={assets}
        control={control}
        formFieldName="isBidAssetNft"
        setValue={setValue}
        setValueLabel="bidAssetId"
      >
        <>
          {isBidAssetNft && (
            <Stack css={{ width: "100%" }}>
              <ControlledField
                control={control}
                name="bidNFTAssetId"
                label="Bid NFT Asset Id"
                isRequired
                isInvalid={Boolean(formState.errors.bidNFTAssetId)}
                render={({ field }) => (
                  <Input>
                    <Input.Field
                      {...field}
                      aria-label="Bid nft asset id"
                      placeholder="0x000...000"
                    />
                  </Input>
                )}
              />
            </Stack>
          )}
        </>
      </DropdownContainerForm>
    </Stack>
  );
};
