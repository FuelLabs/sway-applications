import { Input, Button } from "@fuel-ui/react";
import type { ChangeEvent } from "react";

interface Props {
  assets: {
    assetId: string;
    assetAmount: string;
  }[];
  onAssetIdChange: (
    event: ChangeEvent<HTMLInputElement>,
    assetIdx: number
  ) => void;
  onAssetAmountChange: (
    event: ChangeEvent<HTMLInputElement>,
    assetIdx: number
  ) => void;
  onAddAsset: () => void;
  onRemoveAsset: (assetIdx: number) => void;
}

export const AssetInputContainer = (props: Props) => {
  return (
    <>
      {props.assets.map((asset, i) => (
        <div key={i}>
          <Input css={{ alignSelf: "stretch" }}>
            <Input.Field
              id={`assetId${i}`}
              name={`assetId${i}`}
              placeholder={`Asset ${i} Id`}
              value={asset.assetId}
              type="text"
              onChange={(e) => props.onAssetIdChange(e, i)}
              css={{ font: "$sans" }}
              aria-label={`Asset input ${i}`}
            />
          </Input>
          <Input css={{ alignSelf: "stretch" }}>
            <Input.Number
              id={`assetAmount${i}`}
              name={`assetAmount${i}`}
              placeholder={`Asset ${i} Amount`}
              value={asset.assetAmount}
              inputMode="decimal"
              onChange={(e) => props.onAssetAmountChange(e, i)}
              aria-label={`Asset amount input ${i}`}
            />
            <Input.ElementRight>
              <Button
                color="tomato"
                leftIcon="Minus"
                onPress={() => props.onRemoveAsset(i)}
              />
            </Input.ElementRight>
          </Input>
        </div>
      ))}
      <Button
        leftIcon="Plus"
        css={{ font: "$sans", width: "50%" }}
        onPress={props.onAddAsset}
        aria-label="Add asset"
      >
        Add Asset
      </Button>
    </>
  );
};
