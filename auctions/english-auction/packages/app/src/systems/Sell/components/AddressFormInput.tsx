import { Flex, Input, Button } from "@fuel-ui/react";
import { useState, useEffect } from "react";
import type { Control, FormState } from "react-hook-form";

import type { CreateAuctionFormValues } from "../hooks/useCreateAuctionForm";

import { getSlicedAddress } from "~/systems/Core";
import { ControlledField } from "~/systems/Core/components/ControlledField";

interface AddressFormInputProps {
  control: Control<CreateAuctionFormValues>;
  formState: FormState<CreateAuctionFormValues>;
  walletAddress: string;
}

export const AddressFormInput = ({
  control,
  formState,
  walletAddress,
}: AddressFormInputProps) => {
  const [slicedWalletAddress, setSlicedWalletAddress] = useState(
    getSlicedAddress(walletAddress)
  );

  useEffect(() => {
    setSlicedWalletAddress(getSlicedAddress(walletAddress));
  }, [walletAddress]);

  return (
    <ControlledField
      control={control}
      name="sellerAddress"
      label="Seller Address"
      isRequired
      isInvalid={Boolean(formState.errors.sellerAddress)}
      render={({ field }) => (
        <Flex>
          <Flex grow={2}>
            <Input css={{ width: "100%", marginRight: "$2" }}>
              <Input.Field
                {...field}
                aria-label="Seller address"
                placeholder="0x000...000"
              />
            </Input>
          </Flex>
          <Flex align="start" css={{ marginTop: "$1", marginRight: "$2" }}>
            <Button
              name="Fill seller address"
              aria-label="Fill seller address"
              onPress={() => field.onChange(walletAddress)}
            >
              {slicedWalletAddress}
            </Button>
          </Flex>
        </Flex>
      )}
    />
  );
};
