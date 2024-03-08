import { Stack, Typography } from "@mui/material"

import { useGetPlayers } from "../hooks";

import { ConnectButton } from ".";

export const ConnectionInfo = () => {
    const players = useGetPlayers();

    return (
        <Stack spacing={1}>
          <ConnectButton />
          {players.length === 0 ? (
            null 
          ) : (
            <>
                <Typography>{`Player 1: ${shortAddress(players[0])}`}</Typography>
                <Typography>{`Player 2: ${shortAddress(players[1])}`}</Typography>
            </>
          )}
        </Stack>
    )
}

const shortAddress = (address: string) => {
    return `${address.slice(0, 8)}...${address.slice(-4)}`
}
