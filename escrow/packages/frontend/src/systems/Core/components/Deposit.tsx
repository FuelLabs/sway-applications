import { Button } from "@fuel-ui/react";
import type { BigNumberish } from "fuels";
import { bn } from "fuels";
import type { ChangeEvent } from "react";
import { useState } from "react";

import { useDeposit } from "../hooks/useDeposit";

import { AssetInput } from "./AssetInput";

interface Props {
  escrowId: BigNumberish;
}

export function Deposit(props: Props) {
  const [assetAmount, setAssetAmount] = useState("");
  const [assetId, setAssetId] = useState("");
  const depositMutation = useDeposit({
    depositAmount: assetAmount,
    depositAsset: assetId,
    escrowId: bn(props.escrowId),
  });

  const handleAssetAmountChange = (event: ChangeEvent<HTMLInputElement>) => {
    const newAssetAmount = event.target.value;
    setAssetAmount(newAssetAmount);
  };

  const handleAssetIdChange = (event: ChangeEvent<HTMLInputElement>) => {
    const newAssetId = event.target.value;
    setAssetId(newAssetId);
  };

  return (
    <>
      <AssetInput
        asset={{ assetId, assetAmount }}
        onAssetAmountChange={handleAssetAmountChange}
        onAssetIdChange={handleAssetIdChange}
      />
      <Button
        aria-label="Deposit"
        isDisabled={depositMutation.isLoading}
        isLoading={depositMutation.isLoading}
        onPress={() => depositMutation.mutate()}
        css={{ margin: "10px" }}
      >
        Deposit Asset
      </Button>
    </>
  );
}
