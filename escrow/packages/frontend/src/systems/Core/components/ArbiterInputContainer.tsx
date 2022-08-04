import { Button, Input } from "@fuel-ui/react";
import { BigNumberish } from "fuels";
import type { ChangeEvent } from "react";

interface Props {
    onArbiterAddressChange: (event: ChangeEvent<HTMLInputElement>) => void;
    onAssetIdChange: (event: ChangeEvent<HTMLInputElement>) => void;
    onFeeChange: (event: ChangeEvent<HTMLInputElement>) => void;
    arbiterAddress: string;
    asset: string;
    feeAmount: BigNumberish | undefined;
}

export const ArbiterInputContainer = (props: Props) => {
    return (
        <>
            <Input css={{ alignSelf: "stretch" }} >
                <Input.Field
                    id={`arbiterAddress`}
                    name={`arbiterAddress`}
                    placeholder={`Arbiter Address`}
                    value={props.arbiterAddress}
                    type="text"
                    onChange={(e) => props.onArbiterAddressChange(e)}
                    css={{ font: "$sans" }}
                />
            </Input>
            <Input css={{ alignSelf: "stretch" }} >
                <Input.Field
                    id={`arbiterAsset`}
                    name={`arbiterAsset`}
                    placeholder={`Asset Id for Arbiter Payment`}
                    value={props.asset}
                    type="text"
                    onChange={(e) => props.onAssetIdChange(e)}
                    css={{ font: "$sans" }}
                />
            </Input>
            <Input css={{ alignSelf: "stretch" }} >
                <Input.Field
                    id={`arbiter`}
                    name={`arbiter`}
                    placeholder={`Amount to pay the Arbiter`}
                    value={props.feeAmount}
                    type="text"
                    onChange={(e) => props.onFeeChange(e)}
                    css={{ font: "$sans" }}
                />
            </Input>
        </>
    );
}