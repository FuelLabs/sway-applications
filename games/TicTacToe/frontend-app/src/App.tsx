import { Container, Typography, Stack, CssBaseline, Box } from "@mui/material";
import toast, { Toaster } from "react-hot-toast";
import { useProvider } from "@fuels/react";
import { Board, ConnectionInfo, NewGameButton } from "./components";
import { useGetGameState } from "./hooks";
import { useAppContext } from "./components";
import { PROVIDER_URL } from "./config";
import { useEffect } from "react";

function App() {
  const { gameState } = useGetGameState();
  const appContext = useAppContext();
  const { provider, isLoading } = useProvider();


  useEffect(() => {
    if (!isLoading && provider && provider.url !== PROVIDER_URL) {
      toast.error(`Your wallet is not connected to the correct network.  Please connect to ${PROVIDER_URL}`);
    }
  }, [provider, isLoading]);

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
          {gameState === "Ended" && <NewGameButton />}
          {(appContext?.showGameBoard || gameState === "Playing") && <Board />}
        </Stack>
      </Container>
      <Toaster />
    </>
  );
}

export default App;
