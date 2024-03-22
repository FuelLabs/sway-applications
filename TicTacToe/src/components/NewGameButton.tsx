import { Button, Typography } from '@mui/material';
import { Address } from 'fuels';

import { useGetPlayers, useNewGame } from '../hooks';

export const NewGameButton = () => {
  const { players } = useGetPlayers();
  const hasPlayers = players.length === 2;
  const newGame = useNewGame(
    hasPlayers ? Address.fromString(players[0]).toHexString() : '',
    hasPlayers ? Address.fromString(players[1]).toHexString() : ''
  );

  if (!hasPlayers) {
    return (
      <Typography sx={{ marginRight: '20px' }}>
        Connect your wallet to start a new game.
      </Typography>
    );
  }

  return (
    <Button
      variant="outlined"
      sx={{ marginRight: '20px' }}
      onClick={() => {
        newGame.mutate();
      }}
    >
      New Game
    </Button>
  );
};
