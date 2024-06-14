import { FuelLogo } from "@/components/FuelLogo";
import { HomeCard } from "@/components/HomeCard";
import { Box, Grid, Stack } from "@mui/material";

export default function Home() {
  return (
    <Stack>
      <nav
        className="flex justify-between items-center p-4 bg-black text-white gap-6 gradient-border
            bg-gradient-to-b
            from-zinc-900
            to-zinc-950/80"
      >
        <Box
          display="flex"
          alignContent="center"
          flexWrap="wrap"
          justifyContent="center"
          alignItems="center"
          paddingTop="4px"
          paddingBottom="4px"
          width="stretch"
        >
          <FuelLogo />
          <div className="text-3xl text-whit font-sans">Sway Applications</div>
        </Box>
      </nav>
      <div className="min-h-screen items-center p-20 flex flex-col gap-6">
        <Grid container spacing={2}>
          <Grid item>
            <HomeCard href="/nft" title="NFT">
              Create and mint NFTs.
            </HomeCard>
          </Grid>
          <Grid item>
            <HomeCard href="/tictactoe" title="TicTacToe">
              Play the classic game of Tic Tac Toe on the Fuel network.
            </HomeCard>
          </Grid>
        </Grid>
      </div>
    </Stack>
  );
}
