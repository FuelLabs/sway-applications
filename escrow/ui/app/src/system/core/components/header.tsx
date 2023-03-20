import {
  Button,
  BoxCentered,
  Dropdown,
  Flex,
  FuelLogo,
  Heading,
  Text,
} from "@fuel-ui/react";
import { useNavigate } from "react-router-dom";
import { WalletState } from "./wallet_state";

export function Header() {
  const navigate = useNavigate();

  return (
    <Flex css={{ height: "$20", background: "transparent" }}>
      <BoxCentered>
        <FuelLogo css={{ marginLeft: "$3" }} />
      </BoxCentered>

      <BoxCentered
        css={{
          width: "100%",
          justifyContent: "flex-start",
        }}
      >
        <Heading
          as="h2"
          css={{
            marginTop: "auto",
            marginBottom: "auto",
            marginLeft: "$2",
            textShadow:
              "-1px -1px 0 #000, 0 -1px 0 #000, 1px -1px 0 #000, 1px 0 0 #000, 1px  1px 0 #000, 0 1px 0 #000, -1px 1px 0 #000, -1px 0 0 #000",
          }}
        >
          Escrow
        </Heading>
      </BoxCentered>

      <BoxCentered gap="$2" css={{ paddingRight: "20px" }}>
        <Dropdown>
          <Dropdown.Trigger>
            <Button
              variant="ghost"
              css={{
                background: "rgb(34 196 53)",
                color: "$blackA12",
                fontWeight: "$semibold",
                border: "1px solid black",
              }}
            >
              Navigation
            </Button>
          </Dropdown.Trigger>

          <Dropdown.Menu
            onAction={(key) => {
              navigate(key as string);
            }}
            css={{ background: "rgb(34 196 53)" }}
          >
            <Dropdown.MenuItem key="create-escrow" textValue="create-escrow">
              <TextComponent text="Create Escrow" />
            </Dropdown.MenuItem>

            <Dropdown.MenuItem key="manage-escrows" textValue="manage-escrows">
              <TextComponent text="Manage Escrows" />
            </Dropdown.MenuItem>
          </Dropdown.Menu>
        </Dropdown>

        <WalletState />
      </BoxCentered>
    </Flex>
  );
}

interface TextDropdownInterface {
  text: string;
}

function TextComponent({ text }: TextDropdownInterface) {
  return (
    <Text
      css={{
        color: "$blackA12",
        fontWeight: "$semibold",
        textAlign: "center",
        marginLeft: "auto",
        marginRight: "auto",
      }}
    >
      {text}
    </Text>
  );
}
