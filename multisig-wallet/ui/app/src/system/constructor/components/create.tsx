import { BoxCentered, Button, Flex, Heading, toast, Stack } from "@fuel-ui/react";
import { useState } from "react";
import { useContract } from "../../core/hooks";
import { NewUserComponent } from "./new_user";
import { UserInput } from "../../../contracts/MultisigContractAbi";

export function CreateWallet() {
    // Used for our component listeners
    const [users, setUsers] = useState<UserInput[]>([])

    const [userComponents, setUserComponents] = useState([<NewUserComponent onChange={createUser} id={1} />])
    const { contract, isLoading, isError } = useContract()

    async function useConstructor() {
        // const userAddress = document.querySelector<HTMLInputElement>(
        //     `[name="create-recipient"]`
        // )!.value;

        // const userWeight = document.querySelector<HTMLInputElement>(
        //     `[name="create-weight"]`
        // )!.value;

        // let user: UserInput = {
        //     address: userAddress,
        //     weight: userWeight
        // }

        console.log(userComponents);

        // await contract!.functions.constructor([user]).call();
        // await contract!.functions.constructor([]).call();
        toast.success("Wallet created!", { duration: 10000 });
    }

    async function createUser(address: string, weight: number) {
        let userArray: UserInput[] = [];

        for (let user of users) {
            userArray.push(user);
        }

        let user: UserInput = { address, weight };
        setUsers([...userArray, user ]);
        // setUsers([...users, user ]);
    }

    async function addUser() {
        setUserComponents([...userComponents, <NewUserComponent onChange={createUser} id={userComponents.length+1} /> ]);
    }

    async function removeUser() {
        if (userComponents.length === 1) {
            toast.error("Cannot remove the last user")
            return;
        }

        setUserComponents([...userComponents.splice(0, userComponents.length - 1)]);
    }

    return (
        <BoxCentered css={{ marginTop: "12%", width: "50%" }}>
            <Stack css={{ minWidth: "100%" }}>
                <Heading as="h3" css={{ marginLeft: "auto", marginRight: "auto", marginBottom: "$10", color: "$accent1"}}>
                    Create a new wallet
                </Heading>

                {userComponents.map((userComponent, index) => userComponent)}

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
