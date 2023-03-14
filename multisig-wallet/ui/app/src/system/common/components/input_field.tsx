import { Input, Text } from "@fuel-ui/react";
import { useIsConnected } from "../../core/hooks";

interface InputFieldInput {
  onChange: (address: string) => void;
  placeholder: string;
  text: string;
}

export const InputFieldComponent = ({
  onChange,
  placeholder,
  text,
}: InputFieldInput) => {
  const [isConnected] = useIsConnected();

  return (
    <>
      <Text color="blackA12" css={{ fontWeight: "$semibold" }}>
        {text}
      </Text>
      <Input
        isDisabled={!isConnected}
        size="lg"
        css={{ marginBottom: "$1", boxShadow: "1px 1px 5px 2px grey" }}
      >
        <Input.Field
          onChange={(event) => onChange(event.target.value)}
          placeholder={placeholder}
        />
      </Input>
    </>
  );
};
