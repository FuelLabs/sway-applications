import { Button } from "@mui/material";

import { useIsConnected, useConnect, useDisconnect } from "@fuels/react";

export const ConnectButton = () => {
    const { isConnected } = useIsConnected();
    const { connect, isLoading: isConnectLoading } = useConnect();
    const { disconnect, isLoading: isDisconnectLoading } = useDisconnect();

    function getButtonText() {
        if (isConnectLoading) {
            return "Connecting...";
        }
        if (isDisconnectLoading) {
            return "Disconnecting...";
        }
        if (isConnected) {
            return "Disconnect";
        }
        return "Connect";
    }

    return (
        <Button variant="outlined" sx={{ borderColor: "green", color: "green" }} onClick={() => {
            if (isConnected) {
                disconnect();
            } else {
                connect();
            }
        }}>{getButtonText()}</Button>
    );
}
