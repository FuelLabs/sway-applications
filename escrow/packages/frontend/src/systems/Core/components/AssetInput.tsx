import { Input } from "@fuel-ui/react";
import type { ChangeEvent } from "react";

interface Props {
  asset: { assetId: string; assetAmount: string };
  onAssetIdChange: (event: ChangeEvent<HTMLInputElement>) => void;
  onAssetAmountChange: (event: ChangeEvent<HTMLInputElement>) => void;
}

export function AssetInput(props: Props) {
  return (
    <>
      <Input css={{ alignSelf: "stretch" }}>
        <Input.Field
          id={`assetId`}
          name={`assetId`}
          placeholder={`Asset Id`}
          value={props.asset.assetId}
          type="text"
          onChange={(e) => props.onAssetIdChange(e)}
          css={{ font: "$sans" }}
        />
      </Input>
      <Input css={{ alignSelf: "stretch" }}>
        <Input.Number
          id={`assetAmount`}
          name={`assetAmount`}
          placeholder={`Asset Amount`}
          value={props.asset.assetAmount}
          inputMode="decimal"
          onChange={(e) => props.onAssetAmountChange(e)}
        />
      </Input>
    </>
  );
}
