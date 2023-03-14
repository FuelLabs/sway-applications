import { BoxCentered, Button, Heading, toast, Stack } from "@fuel-ui/react";
import { useState } from "react";
import { useContract, useIsConnected } from "../../core/hooks";
import {
  InputFieldComponent,
  InputNumberComponent,
  RadioGroupComponent,
  SignatureButtonComponent,
  SignatureComponent,
} from "../../common/components";
import {
  validateAddress,
  validateContractId,
  validateData,
} from "../../common/utils";
import {
  IdentityInput,
  SignatureInfoInput,
} from "../../../contracts/MultisigContractAbi";

export function ExecuteTransactionPage() {
  const [address, setAddress] = useState("");
  const [assetAmount, setAssetAmount] = useState(0);
  const [data, setData] = useState("");
  const [signatures, setSignatures] = useState<SignatureInfoInput[]>([
    {
      message_format: { None: [] },
      message_prefix: { None: [] },
      signature: { bytes: ["", ""] },
      wallet_type: { Fuel: [] },
    },
  ]);

  const [recipient, setRecipient] = useState("address");
  const { contract, isLoading, isError } = useContract();
  const [isConnected] = useIsConnected();

  async function executeTransaction() {
    let identity: IdentityInput;

    if (recipient === "address") {
      let { address: user, isError } = validateAddress(address);
      if (isError) return;

      identity = { Address: { value: user } };
    } else {
      let { address: user, isError } = validateContractId(address);
      if (isError) return;

      identity = { ContractId: { value: user } };
    }

    const { data: validatedData, isError } = validateData(data);
    if (isError) return;

    await contract!.functions
      .execute_transaction(validatedData, signatures, identity, assetAmount)
      .call()
      .then(
        (success) => {
          toast.success("Transaction complete!", { duration: 10000 });
        },
        (error) => {
          if (error.logs === undefined || error.logs.length === 0) {
            toast.error("Unknown error occurred during contract call.", {
              duration: 10000,
            });
          } else {
            toast.error(`Error: ${Object.keys(error.logs[0])[0]}`, {
              duration: 10000,
            });
          }
        }
      );
  }

  async function updateSignature(index: number, signature: string) {
    const localSignatures = [...signatures];
    // TODO: Figure out how to convert the signed message into a B512 in the SignatureInfo
    localSignatures[index].signature.bytes = [signature, signature];
    setSignatures(localSignatures);
  }

  async function addSignature() {
    let signature: SignatureInfoInput = {
      message_format: { None: [] },
      message_prefix: { None: [] },
      signature: { bytes: ["", ""] },
      wallet_type: { Fuel: [] },
    };
    setSignatures([...signatures, signature]);
  }

  async function removeSignature() {
    if (signatures.length === 1) {
      toast.error("Cannot remove the last signature");
      return;
    }

    setSignatures([...signatures.splice(0, signatures.length - 1)]);
  }

  return (
    <BoxCentered css={{ marginTop: "12%", width: "30%" }}>
      <Stack css={{ minWidth: "100%" }}>
        <Heading
          as="h3"
          css={{
            marginLeft: "auto",
            marginRight: "auto",
            marginBottom: "$10",
            color: "$accent1",
          }}
        >
          Execute a transaction
        </Heading>

        <InputFieldComponent
          onChange={setAddress}
          text="Recipient address"
          placeholder="0x80d5e8c2be..."
        />
        <InputNumberComponent
          onChange={setAssetAmount}
          text="Asset amount"
          placeholder="1.0"
        />
        <InputFieldComponent
          onChange={setData}
          text="Data to sign"
          placeholder="0x252afeeb6e..."
        />

        {signatures.map((signature, index) => {
          return <SignatureComponent handler={updateSignature} index={index} />;
        })}

        <Button
          color="accent"
          onPress={executeTransaction}
          size="lg"
          variant="solid"
          isDisabled={!isConnected}
          css={{
            marginTop: "$2",
            boxShadow: "0px 0px 3px 1px",
            fontWeight: "$semibold",
          }}
        >
          Execute
        </Button>

        <SignatureButtonComponent
          addHandler={addSignature}
          removeHandler={removeSignature}
        />
        <RadioGroupComponent handler={setRecipient} />
      </Stack>
    </BoxCentered>
  );
}
