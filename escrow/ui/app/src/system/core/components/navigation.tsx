import { Link, Stack, Text } from "@fuel-ui/react";

export function Navigation() {
  return (
    <Stack
      gap="$3"
      css={{ marginTop: "20%", marginLeft: "$3", width: "15%", background: "DarkBlue" }}
    >
      <LinkComponent text="Create Escrow" />
      <LinkComponent text="Manage Escrows" />
    </Stack>
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
        border: "1px solid black",
        borderRadius: "8px",
        padding: "$1",
        width: "100%",
        background: "$pink6",
        height: "$8"
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
