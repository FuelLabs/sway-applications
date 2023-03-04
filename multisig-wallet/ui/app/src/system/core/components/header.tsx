import { BoxCentered, Flex, FuelLogo, Heading, Link } from "@fuel-ui/react";
import { WalletState } from "./wallet_state";

export const Header = () => {
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

                <Link href="/create" css={{ color: 'black', fontWeight: 'bolder' }}>
                    Create
                </Link>

                <Link href="/execute" css={{ color: 'black', fontWeight: 'bolder' }}>
                    Execute
                </Link>

                <Link href="/transfer" css={{ color: 'black', fontWeight: 'bolder' }}>
                    Transfer
                </Link>

                <Link href="/update-threshold" css={{ color: 'black', fontWeight: 'bolder' }}>
                    Threshold
                </Link>

                <Link href="/update-weight" css={{ color: 'black', fontWeight: 'bolder' }}>
                    Weight
                </Link>

                <Link href="/view" css={{ color: 'black', fontWeight: 'bolder' }}>
                    View
                </Link>

                <WalletState />

            </BoxCentered>
        </Flex>
    );
}
