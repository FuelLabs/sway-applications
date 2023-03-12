import { Input, Text } from "@fuel-ui/react";
import { useIsConnected } from "../../core/hooks";

interface InputFieldInput {
    onChange: (address: string) => void,
    placeholder: string,
    text: string,
}

export const InputFieldComponent = ({ onChange, placeholder, text }: InputFieldInput) => {
    const [isConnected] = useIsConnected();

    return (
        <>
            <Text color="blackA12">{text}</Text>
            <Input isDisabled={!isConnected} size="lg">
                <Input.Field onChange={(event) => onChange(event.target.value)} placeholder={placeholder} />
            </Input>
        </>
    );
}
