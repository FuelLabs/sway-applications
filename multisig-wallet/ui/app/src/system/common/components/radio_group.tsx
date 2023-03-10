import { Heading, RadioGroup } from "@fuel-ui/react";

interface ComponentInput {
    handler: (recipient: string) => void,
}

export function RadioGroupComponent( { handler }: ComponentInput ) {
    return (
        <>
            <Heading as="h4" css={{ marginLeft: "auto", marginRight: "auto", marginTop: "$8", color: "$accent1" }}>
                Recipient Type
            </Heading>

            <RadioGroup defaultValue="address" direction="row" css={{ margin: "auto", ".fuel_form--label": { color: "$blackA12" } }} >
                <RadioGroup.Item onClick={() => handler("address")} label="Address" value="address" />
                <RadioGroup.Item onClick={() => handler("contract")} label="Contract" value="contract" />
            </RadioGroup>
        </>
    );
}
