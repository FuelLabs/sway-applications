import { Container, Typography, Stack, CssBaseline, Box } from "@mui/material";
import {
  Board,
  ConnectionInfo,
  NewGameButton,
} from "./components";
import { useGetGameState } from "./hooks";
import { useAppContext } from "./components";

function App() {
  const { gameState } = useGetGameState();
  const appContext = useAppContext();
  console.log(`appContext`, appContext);

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
          <Box display="flex" alignItems="center">
            {gameState === "Ended" && <NewGameButton />}
          </Box>
          {(appContext?.showGameBoard || gameState === "Playing") && <Board />}
        </Stack>
      </Container>
    </>
  );
}

export default App;
