import { Card, Heading } from "@fuel-ui/react";

export function CreatedWallet() {
    return (
        <Card css={{ width: '40vw', maxWidth: '450px', height: "12vh", background: 'rgb(63 149 57)', boxShadow: '0px 0px 7px 2px black', marginTop: '15vh' }}>
            <Heading as="h4" css={{ margin: "auto", color: "$blackA12" }}>
                Wallet is already created!
            </Heading>
        </Card>
    );
}
