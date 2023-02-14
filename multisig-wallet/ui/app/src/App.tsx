import { Box, BoxCentered, ButtonLink, Card, CardList, Container, Flex, FuelLogo, Heading, Link, Stack, ThemeProvider, toast } from "@fuel-ui/react";
import { QueryClientProvider } from "@tanstack/react-query";
import { queryClient } from "./utils/queryClient";

function App() {
  async function doNothing() {
    console.log("lalalala")
  }

  async function connect() {
    const isConnected = await window.fuel.isConnected();
    if (!isConnected) {
      await fuel.connect();
      toast.success("Connected!", { duration: 4000 });
    } else {
      toast.error("Fuel wallet is already connected!", { duration: 4000 });
    }
  }

  async function disconnect() {
    const isConnected = await window.fuel.isConnected();
    if (!isConnected) {
      toast.error("Fuel wallet is already disconnected!", { duration: 4000 });
    } else {
      await fuel.disconnect();
      toast.success("Disconnected!", { duration: 4000 });
    }
  }

  return (
    <QueryClientProvider client={queryClient}>
      <ThemeProvider>
        <Stack css={{ background: 'rgb(209 226 237)', height: "100vh" }}>
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

              <ButtonLink href="#" onClick={connect} css={{ color: 'black', fontWeight: 'bolder' }}>
                Connect
              </ButtonLink>

              <ButtonLink href="#" onClick={disconnect} css={{ color: 'black', fontWeight: 'bolder' }}>
                Disconnect
              </ButtonLink>
            </BoxCentered>
          </Flex>

          <Card css={{ width: '70vw', height: "20vh", margin: 'auto' }}>
            <Card.Body css={{ textAlign: 'center', margin: 'auto', color: 'white' }}>
              A multi-signature wallet is a wallet that has multiple owners. 
              In order to execute a transaction, a sufficient number of owners need to sign a transaction. 
              This implementation uses weighted owners which means that certain owners may have more "votes" when it comes to increasing the number of approvals in order to surpass the minimum threshold for execution. 
              This implementation allows owners to be both Fuel and EVM addresses; by additionally supporting signatures over <Link href="https://eips.ethereum.org/EIPS/eip-191"><code>EIP-191</code></Link> formatted messages.
            </Card.Body>
          </Card>

          {/* <Container>
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
                  This application comes with <code>src/App.tsx</code> wrapped in <code>QueryClientProvider</code> which uses the client in <code>utils/queryClient</code>.
                </Card.Body>
              </Card>
            </CardList>
          </Container> */}
        </Stack>
      </ThemeProvider>
    </QueryClientProvider >
  );
}

export default App;
