import { Button, Flex } from "@fuel-ui/react";

interface SignatureButtonInput {
  addHandler: () => void;
  removeHandler: () => void;
}

export const SignatureButtonComponent = ({
  addHandler,
  removeHandler,
}: SignatureButtonInput) => {
  return (
    <Flex gap="$2" css={{ marginTop: "$1" }}>
      <Button
        color="accent"
        onPress={addHandler}
        size="lg"
        variant="solid"
        css={{
          width: "50%",
          boxShadow: "0px 0px 3px 1px",
          fontWeight: "$semibold",
        }}
      >
        Add signature
      </Button>

      <Button
        color="accent"
        onPress={removeHandler}
        size="lg"
        variant="solid"
        css={{
          width: "50%",
          boxShadow: "0px 0px 3px 1px",
          fontWeight: "$semibold",
        }}
      >
        Remove signature
      </Button>
    </Flex>
  );
};
