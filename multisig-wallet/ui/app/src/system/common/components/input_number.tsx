import { Input, Text } from "@fuel-ui/react";
import { useIsConnected } from "../../core/hooks";

interface InputNumberInput {
  onChange: (value: number) => void;
  placeholder: string;
  text: string;
}

export const InputNumberComponent = ({
  onChange,
  placeholder,
  text,
}: InputNumberInput) => {
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
        <Input.Number
          onChange={(event) => onChange(Number(event.target.value))}
          placeholder={placeholder}
        />
      </Input>
    </>
  );
};
