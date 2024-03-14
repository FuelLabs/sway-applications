import { Grid, Stack, Typography } from "@mui/material";

import { Cell, useAppContext } from ".";
import { useGetGameBoard, useGetPlayers } from "../hooks";
import { shortAddress } from "../utils";
import { Address } from "fuels";

export const Board = () => {
  const { gameBoard, isLoading } = useGetGameBoard();
  const appContext = useAppContext();
  const { isPlayer1Turn } = useGetPlayers();

  return (
    <Stack width="100%" alignItems="center" spacing={1}>
      {appContext?.appContextData.lastGameOutcome &&
        (typeof appContext?.appContextData.lastGameOutcome === "boolean" ? (
          <Typography>Draw!</Typography>
        ) : (
          <Typography fontSize={20}>
            {`${!isPlayer1Turn ? "Player 1:" : "Player 2:"} ${shortAddress(
              Address.fromString(
                appContext?.appContextData.lastGameOutcome
              ).toString()
            )} won!`}
          </Typography>
        ))}
      <Grid container spacing={2} sx={{ width: "75%" }}>
        {isLoading && <Typography>Loading...</Typography>}
        {gameBoard && !isLoading && (
          <>
            {[...Array(9)].map((_, i) => {
              return <Cell key={i} boardIndex={i} isPlayer1={gameBoard[i]} />;
            })}
          </>
        )}
      </Grid>
    </Stack>
  );
};
