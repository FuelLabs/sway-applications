import { Input, Text } from "@fuel-ui/react";

interface SignatureInput {
    id: number,
    name: string,
}

export const SignatureComponent = ({id, name}: SignatureInput) => {
    name = name + "-signature";

    return (
        <>
            <Text color="blackA12">Signature: {id}</Text>
            <Input size="lg">
                <Input.Field name={name} placeholder="9c3f5ae085a4..." />
            </Input>
        </>
    );
}
