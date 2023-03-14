import { useEffect, useState } from "react";
import { useFuel } from "./useFuel";

export function useIsConnected() {
  const fuel = useFuel();
  const [isConnected, setIsConnected] = useState(false);

  useEffect(() => {
    async function handleConnection() {
      const isConnected = await fuel.isConnected();
      setIsConnected(isConnected);
    }

    if (fuel) {
      handleConnection();
    }

    fuel?.on(fuel.events.connection, handleConnection);
    return () => {
      fuel?.off(fuel.events.connection, handleConnection);
    };
  }, [fuel]);

  return [isConnected];
}
