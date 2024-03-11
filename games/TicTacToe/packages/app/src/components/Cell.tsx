import { Card, CardActionArea, Grid, Typography } from "@mui/material";
import { useEffect, useState } from "react";
import { useGetPlayers, useMakeMove } from "../hooks";
import { Address } from "fuels";
import { useAppContext } from ".";

type CellProps = {
  playerAddress?: string;
  boardIndex: number;
};

export const Cell = ({ playerAddress, boardIndex }: CellProps) => {
  const [text, setText] = useState<"X" | "O" | null>();
  const makeMove = useMakeMove(boardIndex);
  // TODO we call this 9 times (once for every cell), could be improved
  const { players } = useGetPlayers();
  const appContext = useAppContext();

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
    if (makeMove.data?.logs.length === 1) {
      const { logs } = makeMove.data;
      console.log(`logs`, logs);
      if (logs[0].player) {
        appContext?.setAppContext({
          ...appContext,
          lastGameOutcome: logs[0].player.Address.value,
          isGameBoardEnabled: false
        });
      } else {
        appContext?.setAppContext({ ...appContext, lastGameOutcome: true, isGameBoardEnabled: false });
      }
    }
  }, [makeMove.data]);

  return (
    <Grid item xs={4}>
      <Card variant="outlined">
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
          disabled={text !== null && !appContext?.isGameBoardEnabled}
        >
          <Typography sx={{ fontSize: "150px" }}>{text}</Typography>
        </CardActionArea>
      </Card>
    </Grid>
  );
};
