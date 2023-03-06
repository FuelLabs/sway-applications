import { Flex, Stack } from "@fuel-ui/react";
import { InputFieldComponent } from "../../common/input_field";
import { InputNumberComponent } from "../../common/input_number";

interface NewUserInput {
    id: number,
}

export const NewUserComponent = ({ id }: NewUserInput) => {
    const fieldText = `User address: ${id}`;
    const weightText = `Recipient weight: ${id}`;

    return (
        <>
            <Flex gap="$1">
                <Stack css={{ width: "100%" }}>
                    <InputFieldComponent text={fieldText} placeholder="0x80d5e8c2be..." name="create-recipient" />
                </Stack>

                <Stack css={{ width: "100%" }}>
                    <InputNumberComponent text={weightText} placeholder="2" name="create-weight" />
                </Stack>
            </Flex>
        </>
    );
}
