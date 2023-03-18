import { Button, BoxCentered, Dropdown, Flex, FuelLogo, Heading, Link, Text } from "@fuel-ui/react";
import { WalletState } from "./wallet_state";

export function Header() {

  async function t() {}

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
                css={{ background: "$pink6", color: "pink", fontWeight: "$semibold" }}
              >
              Navigation
            </Button>
          </Dropdown.Trigger>
          
          <Dropdown.Menu css={{ background: "$pink6" }}>
            
            <Dropdown.MenuItem key="create-escrow" textValue="create-escrow">
              <LinkComponent text="Create Escrow" />
            </Dropdown.MenuItem>

            <Dropdown.MenuItem key="manage-escrows" textValue="manage-escrows">
              <LinkComponent text="Manage Escrows" />
            </Dropdown.MenuItem>

          </Dropdown.Menu>

        </Dropdown>

        <WalletState />
      </BoxCentered>
    </Flex>
  );
}

interface LinkInput {
  text: string;
}

function LinkComponent({ text }: LinkInput) {
  let tokens = text.split(" ");

  for (let i = 0; i < tokens.length; i++) {
    tokens[i] = tokens[i][0].toLowerCase() + tokens[i].slice(1);
  }

  const ref = tokens.join("-");

  return (
    <Link
      href={`/${ref}`}
      css={{
        "&:hover": { textDecoration: "none", background: "$pink8" },
        // border: "1px solid black",
        // borderRadius: "8px",
        width: "100%",
        background: "$pink6",
        height: "$12"
      }}
    >
        <Text 
            css={{
                color: "pink",
                fontWeight: "$semibold",
                textAlign: "center",
                marginLeft: "auto",
                marginRight: "auto",
            }}
        >
            {text}
        </Text>
    </Link>
  );
}
