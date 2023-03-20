import { Button, Flex, Input, Stack, Text, toast } from "@fuel-ui/react";
import { useIsConnected } from "../../core/hooks";
import { AssetInput } from "../../../contracts/EscrowContractAbi";
import { AssetButtonComponent, ButtonComponent } from "../../common/components";

interface AssetInterface {
  setAssets: (assets: AssetInput[]) => void;
  setPage: (page: number) => void;
  currentPage: number;
  assets: AssetInput[];
  createEscrow: () => void;
}

export function AssetPage({
  setAssets,
  setPage,
  currentPage,
  assets,
  createEscrow,
}: AssetInterface) {
  const isConnected = useIsConnected();

  async function addAsset() {
    let asset: AssetInput = { amount: 0, id: { value: "" } };
    setAssets([...assets, asset]);
  }

  async function removeAsset() {
    if (assets.length === 1) {
      toast.error("Cannot remove the last asset");
      return;
    }

    setAssets([...assets.splice(0, assets.length - 1)]);
  }

  async function updateAsset(index: number, asset: AssetInput) {
    const localAssets = [...assets];
    localAssets[index] = asset;
    setAssets(localAssets);
  }

  return (
    <Flex
      gap="$5"
      css={{ marginLeft: "auto", marginRight: "auto", width: "90%" }}
    >
      <Stack css={{ width: "75%" }}>
        {assets.map((asset, index) => {
          return (
            <Flex key={`create-asset-${index}`} gap="$3">
              <Stack css={{ width: "100%" }}>
                <Text css={{ fontWeight: "$semibold", color: "$blackA12" }}>
                  Asset Id: {index + 1}
                </Text>

                <Input
                  isDisabled={!isConnected}
                  size="lg"
                  css={{
                    marginBottom: "$1",
                    width: "100%",
                    background: "hsl(128deg 90% 38% / 91%)",
                  }}
                >
                  <Input.Field
                    onChange={(event) =>
                      updateAsset(index, {
                        ...asset,
                        id: { value: event.target.value },
                      })
                    }
                    placeholder="0x80d5e8c2be..."
                  />
                </Input>
              </Stack>

              <Stack css={{ width: "100%" }}>
                <Text css={{ fontWeight: "$semibold", color: "$blackA12" }}>
                  Asset amount: {index + 1}
                </Text>

                <Input
                  isDisabled={!isConnected}
                  size="lg"
                  css={{
                    marginBottom: "$1",
                    width: "100%",
                    background: "hsl(128deg 90% 38% / 91%)",
                  }}
                >
                  <Input.Number
                    onChange={(event) =>
                      updateAsset(index, {
                        ...asset,
                        amount: event.target.value,
                      })
                    }
                    placeholder="1.0"
                  />
                </Input>
              </Stack>
            </Flex>
          );
        })}
      </Stack>

      <Stack css={{ width: "25%", marginTop: "$6" }}>
        <ButtonComponent handler={createEscrow} text="Create escrow" />

        <AssetButtonComponent
          addHandler={addAsset}
          removeHandler={removeAsset}
        />

        <Button
          color="accent"
          onPress={() => setPage(currentPage - 1)}
          size="lg"
          variant="solid"
          isDisabled={!isConnected}
          css={{
            marginTop: "$1",
            fontWeight: "$semibold",
            background: "hsl(128deg 90% 38% / 91%)",
            color: "$blackA12",
            width: "100%",
            border: "1px solid black",
          }}
        >
          Back
        </Button>
      </Stack>
    </Flex>
  );
}
