import { Grid, Stack, Typography } from "@mui/material";

import { Cell } from ".";
import { useGetGameBoard } from "../hooks";
import { useState } from "react";

export const Board = () => {
  const { gameBoard } = useGetGameBoard();
  const [lastGameOutcome, setLastGameOutcome] = useState<
    boolean | string | undefined
  >(undefined);

  console.log(`lastGameOutcome`, lastGameOutcome);

  return (
    <Stack width="100%" alignItems="center">
      {lastGameOutcome &&
        (typeof lastGameOutcome === "boolean" ? (
          <Typography>Draw!</Typography>
        ) : (
          <Typography>{`${lastGameOutcome} won!`}</Typography>
        ))}
      <Grid container spacing={2} sx={{ width: "75%" }}>
        {gameBoard && (
          <>
            {[...Array(9)].map((_, i) => {
              return (
                <Cell
                  key={i}
                  boardIndex={i}
                  playerAddress={gameBoard[i]?.Address?.value}
                  setLastGameOutcome={setLastGameOutcome}
                />
              );
            })}
          </>
        )}
      </Grid>
    </Stack>
  );
};
