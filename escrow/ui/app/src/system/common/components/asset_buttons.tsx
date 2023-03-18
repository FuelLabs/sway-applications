import { Button, Flex } from "@fuel-ui/react";

interface AssetButtonInput {
  addHandler: () => void;
  removeHandler: () => void;
}

export const AssetButtonComponent = ({
  addHandler,
  removeHandler,
}: AssetButtonInput) => {
  return (
    <Flex gap="$1" css={{ marginTop: "$2" }}>
      <Button
        color="accent"
        onPress={addHandler}
        size="lg"
        variant="solid"
        css={{
          width: "100%",
          fontWeight: "$semibold",
          background: "$pink6",
          color: "pink",
        }}
      >
        Add asset
      </Button>

      <Button
        color="accent"
        onPress={removeHandler}
        size="lg"
        variant="solid"
        css={{
          width: "100%",
          fontWeight: "$semibold",
          background: "$pink6",
          color: "pink",
        }}
      >
        Remove asset
      </Button>
    </Flex>
  );
};
