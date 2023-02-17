import { BoxCentered, Card, CardList, Container, FuelLogo, Heading, Link, Stack } from "@fuel-ui/react";

export function InfoPage() {

    return (
        <>
            <Stack>
                <BoxCentered>
                    <FuelLogo />
                </BoxCentered>
                <BoxCentered>
                    <Heading as="h2">
                        Scaffolded Sway Application UI
                    </Heading>
                </BoxCentered>
                <BoxCentered>
                    <Heading as="h3">
                        Installed dependencies
                    </Heading>
                </BoxCentered>
                <Container>
                    <CardList>
                        <Card>
                            <Card.Header>
                                <Heading as="h4">
                                    <Link href="https://github.com/FuelLabs/fuels-ts/" isExternal>
                                        <code>fuels</code>
                                    </Link>
                                </Heading>
                            </Card.Header>
                            <Card.Body>
                                Fuel TypeScript SDK.<br></br>
                                Generated types for contracts of this application are at <code>src/contracts</code>.
                            </Card.Body>
                        </Card>
                        <Card>
                            <Card.Header>
                                <Heading as="h4">
                                    <Link href="https://github.com/FuelLabs/fuels-wallet/tree/master/packages/sdk/" isExternal>
                                        <code>fuels @fuel-wallet/sdk</code>
                                    </Link>
                                </Heading>
                            </Card.Header>
                            <Card.Body>
                                Fuel Wallet SDK.<br></br>
                                Hooks at <code>src/hooks</code> can be used to interact with contracts using the Fuel wallet.
                            </Card.Body>
                        </Card>
                        <Card>
                            <Card.Header>
                                <Heading as="h4">
                                    <Link href="https://github.com/FuelLabs/fuel-ui/tree/master/design-system/react/" isExternal>
                                        <code>fuels @fuel-ui/react</code>
                                    </Link>
                                </Heading>
                            </Card.Header>
                            <Card.Body>
                                Fuel design system React components package.<br></br>
                                Refer to the README of the repository linked above, as well as the{" "}
                                <Link href="https://fuellabs.github.io/fuel-ui/" isExternal>
                                    <code>fuel-ui</code> documentation
                                </Link>{" "}
                                for more information, examples, documentation and usage.<br></br>
                                This application comes with <code>src/App.tsx</code> wrapped in <code>fuel-ui</code>'s <code>ThemeProvider</code> component.
                            </Card.Body>
                        </Card>
                        <Card>
                            <Card.Header>
                                <Heading as="h4">
                                    <Link href="https://github.com/FuelLabs/fuel-ui/tree/master/design-system/css/" isExternal>
                                        <code>fuels @fuel-ui/css</code>
                                    </Link>
                                </Heading>
                            </Card.Header>
                            <Card.Body>
                                Fuel design system styling package.<br></br>
                                This application comes with <code>src/main.tsx</code> importing <code>@fuel-ui/css</code>.
                            </Card.Body>
                        </Card>
                        <Card>
                            <Card.Header>
                                <Heading as="h4">
                                    <Link href="https://tanstack.com/query/latest/" isExternal>
                                        <code>@tanstack/react-query</code>
                                    </Link>
                                </Heading>
                            </Card.Header>
                            <Card.Body>
                                Asynchronous state management tool.<br></br>
                                <Link href="https://tanstack.com/query/latest/docs/react/guides/queries" isExternal>
                                    <code>Queries</code>
                                </Link>{" "}
                                and{" "}
                                <Link href="https://tanstack.com/query/latest/docs/react/guides/mutations" isExternal>
                                    <code>mutations</code>
                                </Link>{" "}
                                are extensively used to interact with contracts.<br></br>
                                This application comes with <code>src/App.tsx</code> wrapped in <code>QueryClientProvider</code> which uses the client in <code>src/utils/queryClient</code>.
                            </Card.Body>
                        </Card>
                    </CardList>
                </Container>
            </Stack>
        </>
    );
}
