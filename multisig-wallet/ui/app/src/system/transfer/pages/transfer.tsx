import { BoxCentered, Stack, toast } from "@fuel-ui/react";
import { useState } from "react";
import { useContract, useIsConnected } from "../../core/hooks";
import {
  ButtonComponent,
  HeadingComponent,
  InputFieldComponent,
  InputNumberComponent,
  RadioGroupComponent,
  SignatureButtonComponent,
  SignatureComponent,
} from "../../common/components";
import {
  addSignature,
  removeSignature,
  updateSignature,
  validateAddress,
  validateContractId,
  validateData,
} from "../../common/utils";
import {
  ContractIdInput,
  IdentityInput,
  SignatureInfoInput,
} from "../../../contracts/MultisigContractAbi";

export function TransferPage() {
  const [address, setAddress] = useState("");
  const [asset, setAsset] = useState("");
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
  const contract = useContract();
  const isConnected = useIsConnected();

  async function transfer() {
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

    let { address: validatedAsset, isError: error } = validateContractId(asset);
    if (error) return;

    const { data: validatedData, isError } = validateData(data);
    if (isError) return;

    let assetId: ContractIdInput = { value: validatedAsset };

    await contract!.functions
      .transfer(assetId, validatedData, signatures, identity, assetAmount)
      .call()
      .then(
        (success) => {
          toast.success("Transfer complete!", { duration: 10000 });
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

  return (
    <BoxCentered css={{ marginTop: "12%", width: "30%" }}>
      <Stack css={{ minWidth: "100%" }}>
        <HeadingComponent text="Execute a transfer" />
        <InputFieldComponent
          onChange={setAddress}
          text="Recipient address"
          placeholder="0x80d5e8c2be..."
        />
        <InputFieldComponent
          onChange={setAsset}
          text="Asset id"
          placeholder="0x0000000000..."
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
          return (
            <SignatureComponent
              key={`transfer-signature-${index}`}
              updateHandler={updateSignature}
              handler={setSignatures}
              signatures={signatures}
              index={index}
            />
          );
        })}

        <ButtonComponent
          handler={transfer}
          isConnected={isConnected}
          text="Transfer"
        />
        <SignatureButtonComponent
          addHandler={() => addSignature(setSignatures, signatures)}
          removeHandler={() => removeSignature(setSignatures, signatures)}
        />
        <RadioGroupComponent handler={setRecipient} />
      </Stack>
    </BoxCentered>
  );
}
