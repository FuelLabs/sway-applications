import { Stack, Typography } from "@mui/material"
import { useAccounts } from "@fuels/react";

import { ConnectButton } from ".";

export const ConnectionInfo = () => {
    const { accounts } = useAccounts();

    return (
        <Stack spacing={1}>
          <ConnectButton />
          {accounts.length === 0 ? (
            null 
          ) : (accounts.length >= 2 ? (
            <>
                <Typography>{`Player 1: ${shortAddress(accounts[0])}`}</Typography>
                <Typography>{`Player 2: ${shortAddress(accounts[1])}`}</Typography>
            </>
          ) : (
            <>
                <Typography>{`Player 1: ${shortAddress(accounts[0])}`}</Typography>
                <Typography>{`Player 2: ${shortAddress(accounts[0])}`}</Typography>
            </>
          ))}
        </Stack>
    )
}

const shortAddress = (address: string) => {
    return `${address.slice(0, 8)}...${address.slice(-4)}`
}
