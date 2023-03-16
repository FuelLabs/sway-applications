import { BoxCentered, toast, Stack } from "@fuel-ui/react";
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
  validateData,
} from "../../common/utils";
import { SignatureInfoInput } from "../../../contracts/MultisigContractAbi";

export function ThresholdPage() {
  const [threshold, setThreshold] = useState(0);
  const [data, setData] = useState("");
  const [signatures, setSignatures] = useState<SignatureInfoInput[]>([
    {
      message_format: { None: [] },
      message_prefix: { None: [] },
      signature: { bytes: ["", ""] },
      wallet_type: { Fuel: [] },
    },
  ]);

  const contract = useContract();
  const isConnected = useIsConnected();

  async function executeThreshold() {
    const { data: validatedData, isError } = validateData(data);
    if (isError) return;

    await contract!.functions
      .set_threshold(validatedData, signatures, threshold)
      .call()
      .then(
        (success) => {
          toast.success("Updated threshold!", { duration: 10000 });
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
        <HeadingComponent text="Change threshold for execution" />
        <InputNumberComponent
          onChange={setThreshold}
          text="Threshold"
          placeholder="8"
        />
        <InputFieldComponent
          onChange={setData}
          text="Data to sign"
          placeholder="0x252afeeb6e..."
        />

        {signatures.map((signature, index) => {
          return (
            <SignatureComponent
              key={`threshold-signature-${index}`}
              updateHandler={updateSignature}
              handler={setSignatures}
              signatures={signatures}
              index={index}
            />
          );
        })}

        <ButtonComponent
          handler={executeThreshold}
          isConnected={isConnected}
          text="Set threshold"
        />
        <SignatureButtonComponent
          addHandler={() => addSignature(setSignatures, signatures)}
          removeHandler={() => removeSignature(setSignatures, signatures)}
        />
      </Stack>
    </BoxCentered>
  );
}
