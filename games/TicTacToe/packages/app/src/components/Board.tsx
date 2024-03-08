import { Grid } from "@mui/material";

import { Cell } from ".";
import { useGetGameBoard } from "../hooks";

export const Board = () => {
  const { gameBoard } = useGetGameBoard();
  console.log(`gameBoard`, gameBoard);

  return (
    <Grid container spacing={2} sx={{ width: "75%" }}>
      {gameBoard && (
        <>
          {[...Array(9)].map((_, i) => {
            return <Cell key={i} boardIndex={i} playerAddress={gameBoard[i]?.Address?.value} />;
          })}
        </>
      )}
    </Grid>
  );
};
