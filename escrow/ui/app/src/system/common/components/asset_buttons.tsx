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
          background: "hsl(128deg 90% 38% / 91%)",
          color: "$blackA12",
          border: "1px solid black",
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
          background: "hsl(128deg 90% 38% / 91%)",
          color: "$blackA12",
          border: "1px solid black",
        }}
      >
        Remove asset
      </Button>
    </Flex>
  );
};
