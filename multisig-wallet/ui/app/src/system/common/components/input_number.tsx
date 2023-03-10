import { Input, Text } from "@fuel-ui/react";

interface InputNumberInput {
    onChange: (value: number) => void,
    placeholder: string,
    text: string,
}

export const InputNumberComponent = ({ onChange, placeholder, text }: InputNumberInput) => {
    return (
        <>
            <Text color="blackA12">{text}</Text>
            <Input size="lg">
                <Input.Number onChange={(event) => onChange(Number(event.target.value))} placeholder={placeholder} />
            </Input>
        </>
    );
}
