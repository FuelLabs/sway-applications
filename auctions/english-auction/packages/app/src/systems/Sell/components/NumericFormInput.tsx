import { Form, Input } from "@fuel-ui/react";

interface NumericFormInputProps {
  onChange: (key: string, value: string) => void;
  formLabel: string;
  formValue: string;
  objKey: string;
  isRequired?: boolean;
  isInvalid?: boolean;
  formErrorMessage?: string;
}

export const NumericFormInput = ({
  onChange,
  formLabel,
  formValue,
  objKey,
  ...props
}: NumericFormInputProps) => {
  return (
    <Form.Control
      isRequired={props.isRequired}
      isInvalid={props.isInvalid}
      css={{ minWidth: "100%" }}
    >
      <Form.Label>{formLabel}</Form.Label>
      <Input>
        <Input.Number
          id={objKey}
          allowNegative={false}
          autoComplete="off"
          inputMode="numeric"
          onChange={(e) => onChange(objKey, e.target.value)}
          placeholder="0"
          value={formValue}
        />
      </Input>
      <Form.ErrorMessage>{props.formErrorMessage}</Form.ErrorMessage>
    </Form.Control>
  );
};
