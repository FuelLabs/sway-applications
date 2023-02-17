import { Card, Heading } from "@fuel-ui/react";

export const PleaseConnect = () => {
    return (
        <Card css={{ width: '40vw', maxWidth: '450px', height: "12vh", background: 'rgb(63 149 57)', boxShadow: '0px 0px 7px 2px black', marginTop: '15vh' }}>
            <Card.Header>
                <Heading as="h4">
                    Wallet Disconnected
                </Heading>
            </Card.Header>
            
            <Card.Body css={{ color: 'white' }}>
                Please connect your wallet!
            </Card.Body>
        </Card>
    );
}
