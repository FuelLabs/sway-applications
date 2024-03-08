import { Container, Typography, Stack, Grid, CssBaseline, Box, Button } from "@mui/material";
import { Cell, ConnectionInfo } from "./components";

function App() {

  return (
  <>
  <CssBaseline />
    <Container>
      <Stack alignItems="center" spacing={2} sx={{ marginTop: "32px" }}>
        <Box display="flex" alignItems="center" justifyContent="space-between" width="100%" flexWrap="wrap">
          <Box width="180px"></Box>
          <Typography align="center" variant="h3">
            TicTacToe
          </Typography>
          <ConnectionInfo />
        </Box>
        <Button variant="outlined">New Game</Button>
        <Grid container spacing={2} sx={{ width: "75%" }}>
          <Cell />
          <Cell />
          <Cell />
          <Cell />
          <Cell />
          <Cell />
          <Cell />
          <Cell />
          <Cell />
        </Grid>
      </Stack>
    </Container>
    </>
  )
}

export default App
