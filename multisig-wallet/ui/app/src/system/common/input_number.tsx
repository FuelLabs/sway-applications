import { Input, Text } from "@fuel-ui/react";

interface InputNumberInput {
    name: string,
    placeholder: string,
    text: string,
}

export const InputNumberComponent = ({name, placeholder, text}: InputNumberInput) => {
    return (
        <>
            <Text color="blackA12">{text}</Text>
            <Input size="lg">
                <Input.Number name={name} placeholder={placeholder} />
            </Input>
        </>
    );
}
