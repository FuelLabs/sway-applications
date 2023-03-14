import { Button } from "@fuel-ui/react";

interface ButtonInput {
  handler: () => void;
  isConnected: boolean;
  text: string;
}

export const ButtonComponent = ({
  handler,
  isConnected,
  text,
}: ButtonInput) => {
  return (
    <Button
      color="accent"
      onPress={handler}
      size="lg"
      variant="solid"
      isDisabled={!isConnected}
      css={{
        marginTop: "$1",
        boxShadow: "0px 0px 3px 1px",
        fontWeight: "$semibold",
      }}
    >
      {text}
    </Button>
  );
};
