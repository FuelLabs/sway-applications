import { Button } from "@mui/material";

import { useNewGame } from "../hooks";

export const NewGameButton = () => {
    const newGame = useNewGame();
    
    return (
        <Button variant="outlined" sx={{ marginRight: "20px" }}>New Game</Button>
    );
};
