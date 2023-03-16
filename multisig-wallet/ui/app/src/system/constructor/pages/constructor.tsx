import {
  BoxCentered,
  Button,
  Flex,
  Heading,
  Input,
  Text,
  toast,
  Stack,
} from "@fuel-ui/react";
import { useState } from "react";
import { useContract } from "../../core/hooks";
import { UserInput } from "../../../contracts/MultisigContractAbi";
import { validateAddress } from "../../common/utils/validate_address";
import { useIsConnected } from "../../core/hooks/useIsConnected";

export function ConstructorPage() {
  const [users, setUsers] = useState<UserInput[]>([{ address: "", weight: 1 }]);
  const contract = useContract();
  const isConnected = useIsConnected();

  async function createMultisig() {
    let error = false;
    users.forEach((user, index) => {
      let { address, isError } = validateAddress(user.address);
      if (isError) {
        error = true;
        toast.error(`Invalid address: User ${index + 1}`, { duration: 10000 });
      }

      user.address = address;
    });

    if (error) return;

    await contract!.functions
      .constructor(users)
      .call()
      .then(
        (success) => {
          toast.success("Wallet created!", { duration: 10000 });
        },
        (error) => {
          if (error.logs === undefined || error.logs.length === 0) {
            toast.error("Unknown error occurred during contract call.", {
              duration: 10000,
            });
          } else {
            toast.error(`Error: ${Object.keys(error.logs[0])[0]}`, {
              duration: 10000,
            });
          }
        }
      );
  }

  async function addUser() {
    let user: UserInput = { address: "", weight: 1 };
    setUsers([...users, user]);
  }

  async function updateUser(index: number, user: UserInput) {
    const localUsers = [...users];
    localUsers[index] = user;
    setUsers(localUsers);
  }

  async function removeUser() {
    if (users.length === 1) {
      toast.error("Cannot remove the last user");
      return;
    }
    setUsers([...users.splice(0, users.length - 1)]);
  }

  return (
    <BoxCentered css={{ marginTop: "12%", width: "50%" }}>
      <Stack css={{ minWidth: "100%" }}>
        <Heading
          as="h3"
          css={{
            marginLeft: "auto",
            marginRight: "auto",
            marginBottom: "$10",
            color: "$accent1",
          }}
        >
          Create a new wallet
        </Heading>

        {users.map((user, index) => {
          return (
            <Flex key={`constructor-user-${index}`} gap="$1">
              <Stack css={{ width: "100%" }}>
                <Text color="blackA12" css={{ fontWeight: "$semibold" }}>
                  User address: {index + 1}
                </Text>
                <Input
                  isDisabled={!isConnected}
                  size="lg"
                  css={{
                    marginBottom: "$1",
                    boxShadow: "1px 1px 5px 2px grey",
                  }}
                >
                  <Input.Field
                    onChange={(event) =>
                      updateUser(index, {
                        ...user,
                        address: event.target.value,
                      })
                    }
                    placeholder="0x80d5e8c2be..."
                  />
                </Input>
              </Stack>

              <Stack css={{ width: "100%" }}>
                <Text color="blackA12" css={{ fontWeight: "$semibold" }}>
                  Recipient weight: {index + 1}
                </Text>
                <Input
                  isDisabled={!isConnected}
                  size="lg"
                  css={{
                    marginBottom: "$1",
                    boxShadow: "1px 1px 5px 2px grey",
                  }}
                >
                  <Input.Number
                    onChange={(event) =>
                      updateUser(index, { ...user, weight: event.target.value })
                    }
                    placeholder="1"
                  />
                </Input>
              </Stack>
            </Flex>
          );
        })}

        <Button
          color="accent"
          onPress={createMultisig}
          size="lg"
          variant="solid"
          isDisabled={!isConnected}
          css={{
            marginLeft: "auto",
            marginRight: "auto",
            marginTop: "$2",
            width: "100%",
            boxShadow: "0px 0px 3px 1px",
            fontWeight: "$semibold",
          }}
        >
          Create wallet
        </Button>

        <Flex gap="$2" css={{ marginTop: "$1" }}>
          <Button
            color="accent"
            onPress={addUser}
            size="lg"
            variant="solid"
            css={{
              width: "50%",
              boxShadow: "0px 0px 3px 1px",
              fontWeight: "$semibold",
            }}
          >
            Add user
          </Button>

          <Button
            color="accent"
            onPress={removeUser}
            size="lg"
            variant="solid"
            css={{
              width: "50%",
              boxShadow: "0px 0px 3px 1px",
              fontWeight: "$semibold",
            }}
          >
            Remove user
          </Button>
        </Flex>
      </Stack>
    </BoxCentered>
  );
}
