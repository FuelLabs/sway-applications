import { Input, Text } from "@fuel-ui/react";

interface InputFieldInput {
    onChange: (address: string) => void,
    placeholder: string,
    text: string,
}

export const InputFieldComponent = ({ onChange, placeholder, text }: InputFieldInput) => {
    return (
        <>
            <Text color="blackA12">{text}</Text>
            <Input size="lg">
                <Input.Field onChange={(event) => onChange(event.target.value)} placeholder={placeholder} />
            </Input>
        </>
    );
}
