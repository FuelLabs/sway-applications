import { Card, CardActionArea, Grid, Typography } from "@mui/material";
import { useEffect, useState } from "react";
import { useGetPlayers, useMakeMove } from "../hooks";
import { Address } from "fuels";
import { useAppContext } from ".";
import { useAccount } from "@fuels/react";
import { toast } from "react-hot-toast";

type CellProps = {
  isPlayer1?: boolean;
  boardIndex: number;
};

export const Cell = ({ isPlayer1, boardIndex }: CellProps) => {
  const [text, setText] = useState<"X" | "O" | null>();
  const makeMove = useMakeMove(boardIndex);
  // TODO we call this 9 times (once for every cell), could be improved
  const { players, currentPlayer } = useGetPlayers();
  const { account } = useAccount();
  const appContext = useAppContext();

  useEffect(() => {
    if (players.length === 2) {
      if (isPlayer1) {
        setText("X");
      } else if (!isPlayer1 && isPlayer1 !== undefined) {
        setText("O");
      } else {
        setText(null);
      }
    } else {
      setText(null);
    }
  }, [players, isPlayer1]);

  useEffect(() => {
    if (makeMove.data?.logs.length === 1) {
      const { logs } = makeMove.data;
      if (logs[0].player) {
        appContext?.setAppContext({
          ...appContext.appContextData,
          lastGameOutcome: logs[0].player.Address.value,
          isGameBoardEnabled: false,
        });
      } else {
        appContext?.setAppContext({
          ...appContext.appContextData,
          lastGameOutcome: true,
          isGameBoardEnabled: false,
        });
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
            if (
              account &&
              Address.fromString(account).toHexString() !==
                currentPlayer?.Address?.value
            ) {
              toast.error(
                "Error: it is not your turn.  Please switch to the correct player."
              );
            } else {
              makeMove.mutate();
            }
          }}
          disabled={text !== null || !appContext?.appContextData.isGameBoardEnabled}
        >
          <Typography sx={{ fontSize: "150px" }}>{text}</Typography>
        </CardActionArea>
      </Card>
    </Grid>
  );
};
