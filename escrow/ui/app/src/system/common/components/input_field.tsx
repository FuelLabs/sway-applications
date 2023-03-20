import { Input, Stack, Text } from "@fuel-ui/react";
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
  const isConnected = useIsConnected();

  return (
    <Stack>
      <Text css={{ fontWeight: "$semibold", color: "$blackA12" }}>{text}</Text>
      <Input
        isDisabled={!isConnected}
        size="lg"
        css={{
          marginBottom: "$2",
          background: "hsl(128deg 90% 38% / 91%)",
          border: "1px solid black",
        }}
      >
        <Input.Field
          onChange={(event) => onChange(event.target.value)}
          placeholder={placeholder}
        />
      </Input>
    </Stack>
  );
};
