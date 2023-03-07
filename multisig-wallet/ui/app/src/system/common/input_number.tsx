import { Input, Text } from "@fuel-ui/react";

interface InputNumberInput {
    name: string,
    placeholder: string,
    text: string,
    onChange: (value: number) => void
}

export const InputNumberComponent = ({name, placeholder, text, onChange}: InputNumberInput) => {
    return (
        <>
            <Text color="blackA12">{text}</Text>
            <Input size="lg">
                <Input.Number onChange={(event) => onChange(Number(event.target.value))} name={name} placeholder={placeholder} />
            </Input>
        </>
    );
}
