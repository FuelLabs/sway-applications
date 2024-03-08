import { Typography } from "@mui/material";
import { useGetGameState } from "../hooks";

export const GameStateInfo = () => {
    const { gameState } = useGetGameState();

    return (
        <Typography>{gameState === 'Playing' ? "Game in progress" : "Game not started"}</Typography>
    );
}
