import { useEffect, useState } from 'react';

export const useFuelWeb3 = () => {
  // TODO remove eslint disable comment once we have wallet type info
  const windowLocal = window as any; // eslint-disable-line @typescript-eslint/no-explicit-any
  const [error, setError] = useState('');
  // TODO remove eslint disable comment once we have wallet type info
  const [fuelWeb3, setFuelWeb3] = useState<any>(windowLocal.FuelWeb3); // eslint-disable-line @typescript-eslint/no-explicit-any

  useEffect(() => {
    const timeout = setTimeout(() => {
      if (windowLocal.FuelWeb3) {
        setFuelWeb3(windowLocal.FuelWeb3);
      } else {
        setError('FuelWeb3 not detected on the window!');
      }
    }, 500);
    return () => clearTimeout(timeout);
  }, []);

  return [fuelWeb3, error] as const;
};
