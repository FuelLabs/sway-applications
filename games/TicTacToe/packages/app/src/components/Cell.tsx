import { Card, CardActionArea, Grid, Typography } from "@mui/material";
import { useEffect, useState } from "react";
import { useGetPlayers, useMakeMove } from "../hooks";
import { Address } from "fuels";

type CellProps = {
  playerAddress?: string;
  boardIndex: number;
  setLastGameOutcome: (winnerOrDraw: boolean | string) => void;
};

export const Cell = ({
  playerAddress,
  boardIndex,
  setLastGameOutcome,
}: CellProps) => {
  const [text, setText] = useState<"X" | "O" | null>();
  const makeMove = useMakeMove(boardIndex);
  // TODO we call this 9 times (once for every cell), could be improved
  const { players } = useGetPlayers();

  console.log(`makeMove.data`, makeMove.data);

  console.log(`playerAddress`, playerAddress);

  useEffect(() => {
    if (players.length === 2) {
      if (playerAddress === Address.fromString(players[0]).toHexString()) {
        setText("X");
      } else if (
        playerAddress === Address.fromString(players[1]).toHexString()
      ) {
        setText("O");
      } else {
        setText(null);
      }
    } else {
      setText(null);
    }
  }, [playerAddress, players]);

  useEffect(() => {
    if (makeMove.data) {
      const { logs } = makeMove.data;
      console.log(`logs`, logs);
      if (logs.length === 1) {
        setLastGameOutcome(logs[0].Address.value);
      } else if (logs.length === 2) {
        setLastGameOutcome(true);
      }
    }
  }, [makeMove.data, setLastGameOutcome]);

  return (
    <Grid item xs={4}>
      <Card
        variant="outlined"
      >
        <CardActionArea
          sx={{
            height: "150px",
            alignItems: "center",
            justifyContent: "center",
            display: "flex",
          }}
          onClick={() => {
            makeMove.mutate();
          }}
          disabled={text !== null}
        >
          <Typography sx={{ fontSize: "150px" }}>{text}</Typography>
        </CardActionArea>
      </Card>
    </Grid>
  );
};
