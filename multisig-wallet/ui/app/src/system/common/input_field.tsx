import { Input, Text } from "@fuel-ui/react";

interface InputFieldInput {
    name: string,
    placeholder: string,
    text: string,
    onChange: (address: string) => void
}

export const InputFieldComponent = ({name, placeholder, text, onChange}: InputFieldInput) => {
    return (
        <>
            <Text color="blackA12">{text}</Text>
            <Input size="lg">
                <Input.Field onChange={(event) => onChange(event.target.value)} name={name} placeholder={placeholder} />
            </Input>
        </>
    );
}
