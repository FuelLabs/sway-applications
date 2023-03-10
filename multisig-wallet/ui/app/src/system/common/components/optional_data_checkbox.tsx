import { BoxCentered, Checkbox, Form } from "@fuel-ui/react";

interface ComponentInput {
    handler: (optionalData: boolean) => void,
    optionalData: boolean
}

export function OptionalCheckBoxComponent( { handler, optionalData }: ComponentInput ) {
    return (
        <>
            <BoxCentered css={{ marginTop: "$8" }}>
                <Form.Control css={{ flexDirection: 'row' }}>
                    <Checkbox onClick={() => handler(!optionalData)} id="optional-data"/>
                    <Form.Label htmlFor="optional-data" css={{ color: "$blackA12" }}>
                        Optional data
                    </Form.Label>
                </Form.Control>
            </BoxCentered>
        </>
    );
}
