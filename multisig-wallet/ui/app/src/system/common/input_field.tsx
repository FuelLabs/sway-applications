import { Input, Text } from "@fuel-ui/react";

interface InputFieldInput {
    name: string,
    placeholder: string,
    text: string,
}

export const InputFieldComponent = ({name, placeholder, text}: InputFieldInput) => {
    return (
        <>
            <Text color="blackA12">{text}</Text>
            <Input size="lg">
                <Input.Field name={name} placeholder={placeholder} />
            </Input>
        </>
    );
}
