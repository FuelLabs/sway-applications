import { Heading, RadioGroup } from "@fuel-ui/react";

interface ComponentInput {
    setRecipient: (recipient: string) => void,
}

export function RadioGroupComponent( { setRecipient }: ComponentInput ) {
    return (
        <>
            <Heading as="h4" css={{ marginLeft: "auto", marginRight: "auto", marginTop: "$8", color: "$accent1" }}>
                Recipient Type
            </Heading>

            <RadioGroup defaultValue="address" direction="row" css={{ margin: "auto" }}>
                {/* 
                    TODO: 
                        change labels to be the color black
                        increase the size of the buttons and text 
                */}
                <RadioGroup.Item onClick={() => setRecipient("address")} label="Address" value="address" />
                <RadioGroup.Item onClick={() => setRecipient("contract")} label="Contract" value="contract" />
            </RadioGroup>
        </>
    );
}
