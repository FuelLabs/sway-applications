import { Heading, Stack, toast } from "@fuel-ui/react";
import { useState } from "react";
import { useContract, useIsConnected } from "../../core/hooks";
import {
  ButtonComponent,
  InputFieldComponent,
  InputNumberComponent,
} from "../../common/components";
import { validateAddress, validateData } from "../../common/utils";
import { UserInput } from "../../../contracts/MultisigContractAbi";

export function WeightHashComponent() {
  const [address, setAddress] = useState("");
  const [weight, setWeight] = useState(0);
  const [nonce, setNonce] = useState(0);
  const [data, setData] = useState("");

  const contract = useContract();
  const isConnected = useIsConnected();

  async function getHash() {
    let { address: userAddress, isError: error } = validateAddress(address);
    if (error) return;

    const { data: validatedData, isError } = validateData(data);
    if (isError) return;

    let user: UserInput = {
      address: userAddress,
      weight: weight,
    };

    const { value } = await contract!.functions
      .weight_hash(validatedData, nonce, user)
      .get()
      .then(null, (error) => {
        if (error.logs === undefined || error.logs.length === 0) {
          toast.error("Unknown error occurred during contract call.", {
            duration: 10000,
          });
        } else {
          toast.error(`Error: ${Object.keys(error.logs[0])[0]}`, {
            duration: 10000,
          });
        }
        return;
      });

    toast.success(`Hash: ${value}`, { duration: 10000 });
  }

  return (
    <>
      <Stack>
        <Heading
          as="h4"
          css={{ marginLeft: "auto", marginRight: "auto", color: "$accent1" }}
        >
          Hash for user weight
        </Heading>

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
        <InputNumberComponent
          onChange={setNonce}
          text="Nonce"
          placeholder="3"
        />
        <InputFieldComponent
          onChange={setData}
          text="Data to sign"
          placeholder="0x252afeeb6e..."
        />
        <ButtonComponent
          handler={getHash}
          isConnected={isConnected}
          text="Create hash"
        />
      </Stack>
    </>
  );
}
