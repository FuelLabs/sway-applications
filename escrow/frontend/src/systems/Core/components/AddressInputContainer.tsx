import { Button, Input } from "@fuels-ui/react";
import { InputElementRight } from "@fuels-ui/react/src/components/Input/InputElement";
import { InputField } from "@fuels-ui/react/src/components/Input/InputField";
import type { ChangeEvent } from "react";

interface Props {
    onUserInfoChange: (event: ChangeEvent<HTMLInputElement>, userIdx: number) => void;
    onAddUser: (event: any) => void;
    onRemoveUser: (userId: number) => void;
    users: string[];
}

export const AddressInputContainer = (props: Props) => {
    return (
        <>
            {props.users.map((user, i) => (
                <Input css={{ alignSelf: "stretch" }} >
                    <InputField
                        id={`user${i}`}
                        name={`user${i}`}
                        placeholder={`User ${i} Address`}
                        value={user}
                        type="text"
                        onChange={(e) => props.onUserInfoChange(e, i)}
                        css={{ font: "$sans" }}
                    />
                    <InputElementRight>
                        <Button color="tomato" leftIcon="DividerHorizontalIcon" onPress={() => props.onRemoveUser(i)} />
                    </InputElementRight>
                </Input>
            ))}
            <Button leftIcon="PlusIcon" css={{ font: "$sans", width: "50%" }} onPress={props.onAddUser}>Add User</Button>
        </>
    );
}