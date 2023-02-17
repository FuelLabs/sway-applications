import { BoxCentered, ButtonLink, Flex, FuelLogo, Heading, toast } from "@fuel-ui/react";
import { WalletState } from "./wallet_state";
import { CreatePage } from "../../create/pages";

export const Header = () => {

    async function doNothing() {
        toast.error("Unimplemented!", { duration: 4000 });
    }

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
                <ButtonLink onClick={CreatePage} css={{ color: 'black', fontWeight: 'bolder' }}>
                    Create
                </ButtonLink>

                <ButtonLink onClick={doNothing} css={{ color: 'black', fontWeight: 'bolder' }}>
                    Execute
                </ButtonLink>

                <ButtonLink onClick={doNothing} css={{ color: 'black', fontWeight: 'bolder' }}>
                    Update
                </ButtonLink>

                <ButtonLink onClick={doNothing} css={{ color: 'black', fontWeight: 'bolder' }}>
                    View
                </ButtonLink>

                <WalletState />

            </BoxCentered>
        </Flex>
    );

}
