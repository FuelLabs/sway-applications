import { useConnectUI, useDisconnect } from "@fuels/react";
import { Button } from "./Button";

type ConnectButtonProps = {
  isLoading?: boolean;
  isConnected: boolean;
};

export const ConnectButton = ({
  isLoading,
  isConnected,
}: ConnectButtonProps) => {
  const { connect } = useConnectUI();
  const { disconnect } = useDisconnect();

  // TODO: connect button blinks
  const buttonText = isLoading
    ? "Loading..."
    : isConnected
      ? "Disconnect"
      : "Connect";

  const onClick = isConnected ? disconnect : connect;

  return (
    <Button disabled={isLoading} onClick={onClick}>
      {buttonText}
    </Button>
  );
};
