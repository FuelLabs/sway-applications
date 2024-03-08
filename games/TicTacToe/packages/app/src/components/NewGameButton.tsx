import { Button } from "@mui/material";

import { useNewGame } from "../hooks";

export const NewGameButton = () => {
    const newGame = useNewGame();
    
    return (
        <Button variant="outlined">New Game</Button>
    );
};
