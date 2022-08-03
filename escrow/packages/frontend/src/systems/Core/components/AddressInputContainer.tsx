import { Button, Input } from "@fuel-ui/react";
import type { ChangeEvent } from "react";

interface Props {
    onArbiterAddressChange: (event: ChangeEvent<HTMLInputElement>) => void;
    arbiter: string;
}

export const AddressInputContainer = (props: Props) => {
    return (
        <Input css={{ alignSelf: "stretch" }} >
            <Input.Field
                id={`arbiter`}
                name={`arbiter`}
                placeholder={`Arbiter Address`}
                value={props.arbiter}
                type="text"
                onChange={(e) => props.onArbiterAddressChange(e)}
                css={{ font: "$sans" }}
            />
        </Input>
    );
}