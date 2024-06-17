import { useIsConnected, useConnectUI, useDisconnect } from '@fuels/react';
import { Button } from '@mui/material';

import { useAppContext } from '.';

export const ConnectButton = () => {
  const { isConnected } = useIsConnected();
  const { connect, isLoading: isConnectLoading } = useConnectUI();
  const { disconnect, isLoading: isDisconnectLoading } = useDisconnect();
  const appContext = useAppContext();

  function getButtonText() {
    if (isConnectLoading) {
      return 'Connecting...';
    }
    if (isDisconnectLoading) {
      return 'Disconnecting...';
    }
    if (isConnected) {
      return 'Disconnect';
    }
    return 'Connect';
  }

  return (
    <Button
      variant="outlined"
      sx={{ borderColor: 'green', color: 'green', width: '65%' }}
      onClick={() => {
        if (isConnected) {
          disconnect();
          appContext?.setAppContext({
            ...appContext.appContextData,
            showGameBoard: false,
          });
        } else {
          connect();
        }
      }}
    >
      {getButtonText()}
    </Button>
  );
};
