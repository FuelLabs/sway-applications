import { BoxCentered, Stack, toast } from "@fuel-ui/react";
import { useState } from "react";
import { useContract, useIsConnected } from "../../core/hooks";
import {
  ButtonComponent,
  HeadingComponent,
  InputFieldComponent,
  InputNumberComponent,
  SignatureButtonComponent,
  SignatureComponent,
} from "../../common/components";
import {
  addSignature,
  removeSignature,
  updateSignature,
  validateAddress,
  validateData,
} from "../../common/utils";
import {
  SignatureInfoInput,
  UserInput,
} from "../../../contracts/MultisigContractAbi";

export function WeightPage() {
  const [address, setAddress] = useState("");
  const [weight, setWeight] = useState(0);
  const [data, setData] = useState("");
  const [signatures, setSignatures] = useState<SignatureInfoInput[]>([
    {
      message_format: { None: [] },
      message_prefix: { None: [] },
      signature: "",
      wallet_type: { Fuel: [] },
    },
  ]);

  const contract = useContract();
  const isConnected = useIsConnected();

  async function executeWeight() {
    let { address: userAddress, isError } = validateAddress(address);
    if (isError) return;

    const { data: validatedData, isError: err } = validateData(data);
    if (err) return;

    let user: UserInput = {
      address: userAddress,
      weight: weight,
    };

    await contract!.functions
      .set_weight(validatedData, signatures, user)
      .call()
      .then(
        (success) => {
          toast.success("Updated user weight!", { duration: 10000 });
        },
        (error) => {
          console.log(error);
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
        <HeadingComponent text="Change approval weight of user" />
        <InputFieldComponent
          onChange={setAddress}
          text="Recipient address"
          placeholder="0x80d5e8c2be..."
        />
        <InputNumberComponent
          onChange={setWeight}
          text="New weight"
          placeholder="2"
        />
        <InputFieldComponent
          onChange={setData}
          text="Data to sign"
          placeholder="0x252afeeb6e..."
        />

        {signatures.map((signature, index) => {
          return (
            <SignatureComponent
              key={`weight-signature-${index}`}
              updateHandler={updateSignature}
              handler={setSignatures}
              signatures={signatures}
              index={index}
            />
          );
        })}

        <ButtonComponent
          handler={executeWeight}
          isConnected={isConnected}
          text="Set weight"
        />
        <SignatureButtonComponent
          addHandler={() => addSignature(setSignatures, signatures)}
          removeHandler={() => removeSignature(setSignatures, signatures)}
        />
      </Stack>
    </BoxCentered>
  );
}
