import { Container, Typography, Stack, CssBaseline, Box } from "@mui/material";
import { Toaster } from "react-hot-toast";
import { useProvider, useIsConnected } from "@fuels/react";
import { Board, ConnectionInfo, NewGameButton } from "./components";
import { useGetGameState } from "./hooks";
import { useAppContext } from "./components";
import { PROVIDER_URL } from "./config";

function App() {
  const { gameState } = useGetGameState();
  const appContext = useAppContext();
  const { provider, isLoading, isError } = useProvider();
  const { isConnected, isLoading: isConnectedLoading } = useIsConnected();

  console.log(`gameState`, gameState);

  const showProviderError =
    (!isLoading && provider && provider.url !== PROVIDER_URL) || isError;

  return (
    <>
      <CssBaseline />
      <Container>
        <Stack alignItems="center" spacing={2} sx={{ marginTop: "32px" }}>
          <Box
            display="flex"
            alignItems="center"
            justifyContent="space-between"
            width="100%"
            flexWrap="wrap"
          >
            <Box width="300px"></Box>
            <Typography align="center" variant="h3">
              TicTacToe
            </Typography>
            <ConnectionInfo />
          </Box>
          {isConnected === false && !isConnectedLoading ? (
            <Typography fontSize="20px">{`Your wallet is not connected to the app.  Please press the connect button and connect your wallet.`}</Typography>
          ) : showProviderError ? (
            <Typography fontSize="20px">{`Your wallet is not connected to the correct network.  Please connect to ${PROVIDER_URL}`}</Typography>
          ) : null}
          {gameState === "Ended" && <NewGameButton />}
          {(appContext?.showGameBoard || gameState === "Playing") && <Board />}
        </Stack>
      </Container>
      <Toaster />
    </>
  );
}

export default App;
