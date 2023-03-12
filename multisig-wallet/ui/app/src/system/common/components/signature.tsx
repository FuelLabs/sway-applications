import { Input, Stack, Text } from "@fuel-ui/react";
import { useIsConnected } from "../../core/hooks/useIsConnected";

interface SignatureInput {
    handler: (index: number, signature: string) => void,
    index: number,
}

export const SignatureComponent = ({ handler, index }: SignatureInput) => {
    const [isConnected] = useIsConnected();

    return (
        <Stack css={{ width: "100%" }}>
            <Text color="blackA12">Signature: {index+1}</Text>
            <Input isDisabled={!isConnected} size="lg">
                <Input.Field onChange={(event) => handler(index, event.target.value)} placeholder="0x7b3f1693f3..." />
            </Input>
        </Stack>
    );
}
