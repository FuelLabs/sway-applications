import { Checkbox, Form, Input } from "@fuel-ui/react";
import { useState } from "react";
import type { Control, FormState } from "react-hook-form";

import type { CreateAuctionFormValues } from "../hooks/useCreateAuctionForm";

import { ControlledField } from "~/systems/Core/components/ControlledField";

interface ReservePriceInputProps {
  control: Control<CreateAuctionFormValues>;
  formState: FormState<CreateAuctionFormValues>;
}

export const ReservePriceInput = ({
  control,
  formState,
}: ReservePriceInputProps) => {
  const [hasReservePrice, setHasReservePrice] = useState(false);

  return (
    <>
      <ControlledField
        control={control}
        name="reservePriceCheckbox"
        render={() => (
          <Form.Control css={{ flexDirection: "row" }}>
            <Checkbox
              onCheckedChange={() => setHasReservePrice(!hasReservePrice)}
            />
            <Form.Label>Set reserve price</Form.Label>
          </Form.Control>
        )}
      />

      {hasReservePrice && (
        <ControlledField
          control={control}
          name="reservePrice"
          label="Reserve Price"
          isInvalid={Boolean(formState.errors.reservePrice)}
          render={({ field }) => (
            <Input>
              <Input.Number
                {...field}
                aria-label="Reserve price"
                placeholder="0.0"
                allowNegative={false}
              />
            </Input>
          )}
        />
      )}
    </>
  );
};
