import { BoxCentered, ButtonLink, Flex, FuelLogo, Heading, toast } from "@fuel-ui/react";
import { WalletState } from "./wallet_state";
import { CreatePage } from "../../create/pages";
import { ExecutePage } from "../../execute/pages";
import { UpdatePage } from "../../update/pages";
import { ViewPage } from "../../view/pages";

export const Header = props => {

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
                <ButtonLink onClick={() => props.setPage(CreatePage)} css={{ color: 'black', fontWeight: 'bolder' }}>
                    Create
                </ButtonLink>

                <ButtonLink onClick={() => props.setPage(ExecutePage)} css={{ color: 'black', fontWeight: 'bolder' }}>
                    Execute
                </ButtonLink>

                <ButtonLink onClick={() => props.setPage(UpdatePage)} css={{ color: 'black', fontWeight: 'bolder' }}>
                    Update
                </ButtonLink>

                <ButtonLink onClick={() => props.setPage(ViewPage)} css={{ color: 'black', fontWeight: 'bolder' }}>
                    View
                </ButtonLink>

                <WalletState />

            </BoxCentered>
        </Flex>
    );

}
