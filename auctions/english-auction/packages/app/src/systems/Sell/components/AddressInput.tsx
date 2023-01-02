import { Flex, Button, toast } from "@fuel-ui/react";
import { useEffect, useState } from "react";

import { IdentityFormInput } from "./IdentityFormInput";

import { useWallet } from "~/systems/Core/hooks/useWallet";
import { getSlicedAddress } from "~/systems/Core/utils";

interface AddressInputProps {
  onChange: (field: string, value: string) => void;
  identityValue: string;
  objKey: string;
  label: string;
}

export const AddressInput = ({
  onChange,
  identityValue,
  objKey,
  label,
}: AddressInputProps) => {
  const wallet = useWallet();

  if (!wallet) toast.error("Wallet not connected");

  const [slicedWalletAddress, setSlicedWalletAddress] = useState(
    wallet && getSlicedAddress(wallet.address.toString()!)
  );

  useEffect(() => {
    setSlicedWalletAddress(
      wallet && getSlicedAddress(wallet.address.toString()!)
    );
  }, [wallet]);

  return (
    <Flex>
      <Flex grow={2}>
        <IdentityFormInput
          onChange={onChange}
          identityValue={identityValue}
          objKey={objKey}
          label={label}
        />
      </Flex>
      <Flex align="start" css={{ marginTop: "$9" }}>
        {wallet && (
          <Button onPress={() => onChange(objKey, wallet.address.toString()!)}>
            {slicedWalletAddress}
          </Button>
        )}
      </Flex>
    </Flex>
  );
};
