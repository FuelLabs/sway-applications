import { BoxCentered, Checkbox, Form } from "@fuel-ui/react";

interface ComponentInput {
    setOptionalData: (optionalData: boolean) => void,
    optionalData: boolean
}

export function OptionalCheckBoxComponent( { setOptionalData, optionalData }: ComponentInput ) {
    return (
        <>
            <BoxCentered css={{ marginTop: "$8" }}>
                <Form.Control css={{ flexDirection: 'row' }}>
                    <Checkbox onClick={() => setOptionalData(!optionalData)} id="optional-data"/>
                    <Form.Label htmlFor="optional-data" css={{ color: "$blackA12" }}>
                        Optional data
                    </Form.Label>
                </Form.Control>
            </BoxCentered>
        </>
    );
}
