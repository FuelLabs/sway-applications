import { Form, Input } from "@fuel-ui/react";

interface IdentityFormInputProps {
  onChange: (key: string, value: string) => void;
  identityValue: string;
  objKey: string;
  label: string;
}

export const IdentityFormInput = ({
  onChange,
  identityValue,
  objKey,
  label,
}: IdentityFormInputProps) => {
  return (
    <Form.Control isRequired css={{ minWidth: "100%" }}>
      <Form.Label>{label}</Form.Label>
      <Input>
        <Input.Field
          id={objKey}
          autoComplete="off"
          onChange={(e) => onChange(objKey, e.target.value)}
          placeholder="0x000...000"
          value={identityValue}
        />
      </Input>
    </Form.Control>
  );
};
