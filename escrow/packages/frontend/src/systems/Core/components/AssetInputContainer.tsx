import type { ChangeEvent } from "react";

import { Input, Button } from "@fuel-ui/react"

interface Props {
    assets: {
        assetId: string;
        assetAmount: string;
    }[];
    onAssetIdChange: (event: ChangeEvent<HTMLInputElement>, assetIdx: number) => void;
    onAssetAmountChange: (event: ChangeEvent<HTMLInputElement>, assetIdx: number) => void;
    onAddAsset: () => void;
    onRemoveAsset: (assetIdx: number) => void;
}

export const AssetInputContainer = (props: Props) => {
    return (
        <>
            {props.assets.map((asset, i) => (
                <>
                    <Input css={{ alignSelf: "stretch" }}>
                        <Input.Field
                            id={`assetId${i}`}
                            name={`assetId${i}`}
                            placeholder={`Asset ${i} Id`}
                            value={asset.assetId}
                            type="text"
                            onChange={(e) => props.onAssetIdChange(e, i)}
                            css={{ font: "$sans" }}
                        />
                    </Input>
                    <Input css={{ alignSelf: "stretch" }}>
                        <Input.Field
                            id={`assetAmount${i}`}
                            name={`assetAmount${i}`}
                            placeholder={`Asset ${i} Amount`}
                            value={asset.assetAmount}
                            type="text"
                            onChange={(e) => props.onAssetAmountChange(e, i)}
                            css={{ font: "$sans" }}
                        />
                        <Input.ElementRight>
                            <Button color="tomato" leftIcon="DividerHorizontalIcon" onPress={() => props.onRemoveAsset(i)} />
                        </Input.ElementRight>
                    </Input>
                </>
            ))}
            <Button leftIcon="PlusIcon" css={{ font: "$sans", width: "50%" }} onPress={props.onAddAsset}>Add Asset</Button>
        </>
    );
}