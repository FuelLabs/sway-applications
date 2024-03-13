import { Stack, Typography } from "@mui/material";

import { useGetPlayers } from "../hooks";

import { ConnectButton } from ".";
import { shortAddress } from "../utils";

export const ConnectionInfo = () => {
  const { players, isPlayer1Turn } = useGetPlayers();

  return (
    <Stack spacing={1} alignItems="end" width="300px">
      <ConnectButton />
      {players.length === 0 ? null : (
        <>
          <Typography>{`${
            isPlayer1Turn ? "Turn ------->" : ""
          } Player 1: ${shortAddress(players[0])}`}</Typography>
          <Typography>{`${
            !isPlayer1Turn && isPlayer1Turn !== undefined ? "Turn ------->" : ""
          } Player 2: ${shortAddress(players[1])}`}</Typography>
        </>
      )}
    </Stack>
  );
};
