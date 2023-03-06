import { BoxCentered, Button, Flex, Heading, toast, Stack } from "@fuel-ui/react";
import { useState } from "react";
import { UserInput } from "../../../contracts/MultisigContractAbi";
import { useContract } from "../../core/hooks";
import { NewUserComponent } from "./new_user";

export function CreateWallet() {
    const [users, setUsers] = useState([<NewUserComponent id={1} />])
    const { contract, isLoading, isError } = useContract()

    async function useConstructor() {
        const userAddress = document.querySelector<HTMLInputElement>(
            `[name="create-recipient"]`
        )!.value;

        const userWeight = document.querySelector<HTMLInputElement>(
            `[name="create-weight"]`
        )!.value;

        let user: UserInput = {
            address: userAddress,
            weight: userWeight
        }

        await contract!.functions.constructor([user]).call();
        toast.success("Wallet created!", { duration: 10000 });
    }

    async function addUser() {
        setUsers([...users, <NewUserComponent id={users.length+1} /> ]);
    }

    async function removeUser() {
        if (users.length === 1) {
            toast.error("Cannot remove the last user")
            return;
        }

        setUsers([...users.splice(0, users.length - 1)]);
    }

    return (
        <BoxCentered css={{ marginTop: "12%", width: "50%" }}>

            <Stack css={{ minWidth: "100%" }}>

                <Heading as="h3" css={{ marginLeft: "auto", marginRight: "auto", marginBottom: "$10", color: "$accent1"}}>
                    Create a new wallet
                </Heading>

                {users.map((userComponent, index) => userComponent)}

                <Button
                    color="accent"
                    onPress={useConstructor}
                    size="lg"
                    variant="solid"
                    css={{ marginLeft: "auto", marginRight: "auto", marginTop: "$2", width: "100%" }}
                >
                    Create wallet
                </Button>

                <Flex gap="$1" css={{ marginTop: "$1" }}>
                    <Button
                        color="accent"
                        onPress={addUser}
                        size="lg"
                        variant="solid"
                        css={{ width: "50%" }}
                    >
                        Add user
                    </Button>

                    <Button
                        color="accent"
                        onPress={removeUser}
                        size="lg"
                        variant="solid"
                        css={{ width: "50%" }}
                    >
                        Remove user
                    </Button>
                </Flex>
            </Stack>
            
        </BoxCentered>
    );
}
