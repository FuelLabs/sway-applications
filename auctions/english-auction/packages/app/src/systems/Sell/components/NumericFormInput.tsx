import { Form, Input } from "@fuel-ui/react";

interface NumericFormInputProps {
    onChange: (key: string, value: string) => void;
    formLabel: string;
    formValue: string;
    objKey: string;
};

export const NumericFormInput = ({ onChange, formLabel, formValue, objKey}: NumericFormInputProps) => {

    return (
        <Form.Control isRequired css={{ minWidth: "100%" }}>
            <Form.Label>{formLabel}</Form.Label>
            <Input>
              <Input.Number
                id={objKey}
                allowNegative={false}
                autoComplete="off"
                inputMode="numeric"
                onChange={(e) => onChange(objKey,  e.target.value)}
                placeholder="0"
                value={formValue}
              />
            </Input>
          </Form.Control>
    );
}