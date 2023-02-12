// @ts-nocheck
import { BoxCentered, Card, CardList, Container, FuelLogo, Heading, Link, Stack, ThemeProvider } from "@fuel-ui/react";
import { QueryClientProvider } from "@tanstack/react-query";
import { queryClient } from "./utils/queryClient";

function App() {
  return (
    <QueryClientProvider client={queryClient}>
      <ThemeProvider>
        <Container size="xl">
          <Stack>
            <BoxCentered><FuelLogo /></BoxCentered>
            <BoxCentered>
              <Heading as="h1">
                Scaffolded Sway Application UI
              </Heading>
            </BoxCentered>
            <Container>
              <Heading as="h2">
                Installed dependencies
              </Heading>
              <CardList>
                <Card>
                  <Card.Header>
                    <Link href="https://github.com/FuelLabs/fuels-ts/" isExternal>
                      <code>fuels</code>
                    </Link>
                  </Card.Header>
                  <Card.Body>
                    Fuel TypeScript SDK<br></br>
                    Generated types for contracts of this application are at <code>src/contracts</code>
                  </Card.Body>
                </Card>
                <Card>
                  <Card.Header>
                    <Link href="https://github.com/FuelLabs/fuels-wallet/tree/master/packages/sdk/" isExternal>
                      <code>fuels @fuel-wallet/sdk</code>
                    </Link>
                  </Card.Header>
                  <Card.Body>
                    Fuel Wallet SDK<br></br>
                    Hooks at <code>src/hooks</code> can be used to interact with contracts using the Fuel wallet</Card.Body>
                </Card>
                <Card>
                  <Card.Header>
                    <Link href="https://github.com/FuelLabs/fuels-wallet/tree/master/packages/types/" isExternal>
                      <code>fuels @fuel-wallet/types</code>
                    </Link>
                  </Card.Header>
                  <Card.Body>

                  </Card.Body>
                </Card>
                <Card>
                  <Card.Header>
                    <Link href="https://github.com/FuelLabs/fuel-ui/tree/master/design-system/react/" isExternal>
                      <code>fuels @fuel-ui/react</code>
                    </Link>
                  </Card.Header>
                  <Card.Body>

                  </Card.Body>
                </Card>
                <Card>
                  <Card.Header>
                    <Link href="https://github.com/FuelLabs/fuel-ui/tree/master/design-system/css/" isExternal>
                      <code>fuels @fuel-ui/css</code>
                    </Link>
                  </Card.Header>
                  <Card.Body>

                  </Card.Body>
                </Card>
                <Card>
                  <Card.Header>
                    <Link href="https://tanstack.com/query/latest/" isExternal>
                      <code>@tanstack/react-query</code>
                    </Link>
                  </Card.Header>
                  <Card.Body>

                  </Card.Body>
                </Card>
              </CardList>
            </Container>
          </Stack>
        </Container>
      </ThemeProvider>
    </QueryClientProvider >
  );
}

export default App;
