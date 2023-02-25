import { Form, Input } from "@fuel-ui/react";
import { DECIMAL_UNITS } from "fuels";

interface AssetAmountInputProps {
  onChange: (key: string, val: string) => void;
  assetAmountLabel: string;
  assetAmountValue: string;
  objKey: string;
}

export const AssetAmountInput = ({
  onChange,
  assetAmountLabel,
  assetAmountValue,
  objKey,
}: AssetAmountInputProps) => {
  // TODO refactor how we handle styles
  return (
    <Form.Control isRequired css={{ minWidth: "100%" }}>
      <Form.Label>{assetAmountLabel}</Form.Label>
      <Input>
        <Input.Number
          id={objKey}
          allowedDecimalSeparators={[".", ","]}
          allowNegative={false}
          autoComplete="off"
          inputMode="decimal"
          decimalScale={DECIMAL_UNITS}
          onChange={(e) => onChange(objKey, e.target.value)}
          placeholder="0.0"
          thousandSeparator={false}
          value={assetAmountValue}
        />
      </Input>
    </Form.Control>
  );
};
