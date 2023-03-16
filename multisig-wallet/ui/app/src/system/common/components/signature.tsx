import { Input, Stack, Text } from "@fuel-ui/react";
import { useIsConnected } from "../../core/hooks";
import { SignatureInfoInput } from "../../../contracts/MultisigContractAbi";

interface SignatureInput {
  handler: (signatures: SignatureInfoInput[]) => void;
  index: number;
  signatures: SignatureInfoInput[];
  updateHandler: (
    index: number,
    signature: string,
    handler: (signatures: SignatureInfoInput[]) => void,
    signatures: SignatureInfoInput[]
  ) => void;
}

export const SignatureComponent = ({
  handler,
  index,
  signatures,
  updateHandler,
}: SignatureInput) => {
  const isConnected = useIsConnected();

  return (
    <Stack css={{ width: "100%" }}>
      <Text color="blackA12" css={{ fontWeight: "$semibold" }}>
        Signature: {index + 1}
      </Text>
      <Input
        isDisabled={!isConnected}
        size="lg"
        css={{ marginBottom: "$1", boxShadow: "1px 1px 5px 2px grey" }}
      >
        <Input.Field
          onChange={(event) =>
            updateHandler(index, event.target.value, handler, signatures)
          }
          placeholder="0x7b3f1693f3..."
        />
      </Input>
    </Stack>
  );
};
