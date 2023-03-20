import { Button } from "@fuel-ui/react";
import { useIsConnected } from "../../core/hooks";

interface ButtonInput {
  handler: () => void;
  text: string;
}

export const ButtonComponent = ({ handler, text }: ButtonInput) => {
  const isConnected = useIsConnected();

  return (
    <Button
      color="accent"
      onPress={handler}
      size="lg"
      variant="solid"
      isDisabled={!isConnected}
      css={{
        marginTop: "$2",
        fontWeight: "$semibold",
        background: "hsl(128deg 90% 38% / 91%)",
        color: "$blackA12",
        border: "1px solid black",
      }}
    >
      {text}
    </Button>
  );
};
