import { Grid, Stack, Typography } from "@mui/material";

import { Cell, useAppContext } from ".";
import { useGetGameBoard } from "../hooks";
import { shortAddress } from "../utils";
import { Address } from "fuels";

export const Board = () => {
  const { gameBoard, isLoading } = useGetGameBoard();
  const appContext = useAppContext();

  return (
    <Stack width="100%" alignItems="center" spacing={1}>
      {appContext?.lastGameOutcome &&
        (typeof appContext?.lastGameOutcome === "boolean" ? (
          <Typography>Draw!</Typography>
        ) : (
          <Typography fontSize={20}>{`${shortAddress(
            Address.fromString(appContext?.lastGameOutcome).toString()
          )} won!`}</Typography>
        ))}
      <Grid container spacing={2} sx={{ width: "75%" }}>
        {isLoading && <Typography>Loading...</Typography>}
        {gameBoard && !isLoading && (
          <>
            {[...Array(9)].map((_, i) => {
              return (
                <Cell
                  key={i}
                  boardIndex={i}
                  playerAddress={gameBoard[i]?.Address?.value}
                />
              );
            })}
          </>
        )}
      </Grid>
    </Stack>
  );
};
