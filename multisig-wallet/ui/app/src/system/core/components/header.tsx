import { BoxCentered, Flex, FuelLogo, Heading, Link } from "@fuel-ui/react";
import { WalletState } from "./wallet_state";

export function Header() {
    return (
        <Flex>
            <BoxCentered css={{ background: 'rgb(63 149 57)', justifyContent: 'flex-start', boxShadow: "11px 1px 8px 0px black", borderBottom: "1px solid black" }}>
                <FuelLogo />
            </BoxCentered>

            <BoxCentered css={{ background: 'rgb(63 149 57)', width: '100%', justifyContent: 'flex-start', boxShadow: "11px 1px 8px 0px black", borderBottom: "1px solid black" }}>
                <Heading as="h3" css={{ marginTop: 'auto', marginBottom: "auto", textShadow: '-1px -1px 0 #000, 0 -1px 0 #000, 1px -1px 0 #000, 1px 0 0 #000, 1px  1px 0 #000, 0 1px 0 #000, -1px 1px 0 #000, -1px 0 0 #000'}}>
                    Multi-Signature Wallet
                </Heading>
            </BoxCentered>

            <BoxCentered css={{ background: 'rgb(63 149 57)', gap: "25px", paddingRight: "20px", justifyContent: 'space-evenly', boxShadow: "11px 1px 8px 0px black", borderBottom: "1px solid black" }}>
                <LinkComponent text="create" />
                <LinkComponent text="execute" />
                <LinkComponent text="transfer" />
                <LinkComponent text="threshold" />
                <LinkComponent text="weight" />
                <LinkComponent text="hash" />
                <LinkComponent text="utils" />
                <WalletState />
            </BoxCentered>
        </Flex>
    );
}

interface LinkInput {
    text: string
}

function LinkComponent( { text }: LinkInput) {
    return (
        <Link href={`/${text}`} css={{ color: 'black', fontWeight: 'bolder', '&:visited': {color: 'black'} }}>
            {text[0].toUpperCase() + text.slice(1)}
        </Link>
    );
}
