import { Input, Text } from "@fuel-ui/react";
import { useIsConnected } from "../../core/hooks";

interface InputNumberInput {
    onChange: (value: number) => void,
    placeholder: string,
    text: string,
}

export const InputNumberComponent = ({ onChange, placeholder, text }: InputNumberInput) => {
    const [isConnected] = useIsConnected();

    return (
        <>
            <Text color="blackA12">{text}</Text>
            <Input isDisabled={!isConnected} size="lg">
                <Input.Number onChange={(event) => onChange(Number(event.target.value))} placeholder={placeholder} />
            </Input>
        </>
    );
}
