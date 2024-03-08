import { Card, Grid, Typography } from "@mui/material";
import { useEffect, useState } from "react";
import { useGetPlayers, useMakeMove } from "../hooks";
import { Address } from "fuels";

type CellProps = {
  playerAddress?: string;
  boardIndex: number;
};

export const Cell = ({ playerAddress, boardIndex }: CellProps) => {
  const [text, setText] = useState<"X" | "O" | null>();
  const makeMove = useMakeMove(boardIndex);
  const { players } = useGetPlayers();

  console.log(`playerAddress`, playerAddress);

  useEffect(() => {
   if (players.length === 2) {
      if (playerAddress === Address.fromString(players[0]).toHexString()) {
         setText("X");
      } else if (playerAddress === Address.fromString(players[1]).toHexString()) {
         setText("O");
      }
   }
  }, [playerAddress, players]);

  return (
    <Grid item xs={4}>
      <Card
        variant="outlined"
        sx={{
          height: "150px",
          alignItems: "center",
          justifyContent: "center",
          display: "flex",
        }}
        onClick={() => makeMove.mutate()}
      >
        <Typography sx={{ fontSize: "150px" }}>{text}</Typography>
      </Card>
    </Grid>
  );
};
