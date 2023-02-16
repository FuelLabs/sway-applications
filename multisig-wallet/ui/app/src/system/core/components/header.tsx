import { BoxCentered, ButtonLink, Flex, FuelLogo, Heading, toast } from "@fuel-ui/react";
import { WalletState } from "./wallet_state";

export const Header = () => {

    async function doNothing() {
        toast.error("Unimplemented!", { duration: 4000 });
    }

    return (
        <Flex css={{ boxShadow: "0px 2px black" }}>
            <BoxCentered css={{ background: 'rgb(63 149 57)', justifyContent: 'flex-start' }}>
                <FuelLogo />
            </BoxCentered>

            <BoxCentered css={{ background: 'rgb(63 149 57)', width: '60%', justifyContent: 'flex-start' }}>
                <Heading as="h3" css={{ marginTop: 'auto', marginBottom: "auto", textShadow: '-1px -1px 0 #000, 0 -1px 0 #000, 1px -1px 0 #000, 1px 0 0 #000, 1px  1px 0 #000, 0 1px 0 #000, -1px 1px 0 #000, -1px 0 0 #000'}}>
                    Multi-Signature Wallet
                </Heading>
            </BoxCentered>

            <BoxCentered css={{ background: 'rgb(63 149 57)', width: '40%', justifyContent: 'space-evenly' }}>
                <ButtonLink href="#" onClick={doNothing} css={{ color: 'black', fontWeight: 'bolder' }}>
                    Create
                </ButtonLink>

                <ButtonLink href="#" onClick={doNothing} css={{ color: 'black', fontWeight: 'bolder' }}>
                    Execute
                </ButtonLink>

                <ButtonLink href="#" onClick={doNothing} css={{ color: 'black', fontWeight: 'bolder' }}>
                    Update
                </ButtonLink>

                <ButtonLink href="#" onClick={doNothing} css={{ color: 'black', fontWeight: 'bolder' }}>
                    View
                </ButtonLink>

                <WalletState />

            </BoxCentered>
        </Flex>
    );

}
