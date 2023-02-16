import { Card, Heading } from "@fuel-ui/react";

export const PleaseConnect = () => {
    return (
        <Card css={{ width: '40vw', height: "12vh", background: 'rgb(63 149 57)' }}>
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
