import { Flex, Stack } from "@fuel-ui/react";
import { InputFieldComponent } from "../../common/input_field";
import { InputNumberComponent } from "../../common/input_number";
import { useEffect, useState } from "react";

interface NewUserInput {
    id: number,
    onChange: (address: string, value: number) => void
}

export const NewUserComponent = ({ id, onChange }: NewUserInput) => {
    // Used for our component listeners
    const [address, setAddress] = useState("")
    const [weight, setWeight] = useState(0)

    // useEffect(() => {
    //     async function main() {
    //         onChange(address, weight);
    //     }
    //     main();
    // }, [address, weight]);

    const fieldText = `User address: ${id}`;
    const weightText = `Recipient weight: ${id}`;

    return (
        <>
            <Flex gap="$1">
                <Stack css={{ width: "100%" }}>
                    <InputFieldComponent onChange={setAddress} text={fieldText} placeholder="0x80d5e8c2be..." name="create-recipient" />
                </Stack>

                <Stack css={{ width: "100%" }}>
                    <InputNumberComponent onChange={setWeight} text={weightText} placeholder="1" name="create-weight" />
                </Stack>
            </Flex>
        </>
    );
}
