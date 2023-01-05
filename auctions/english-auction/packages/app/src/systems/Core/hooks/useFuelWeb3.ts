import { useEffect, useState } from 'react';

export const useFuelWeb3 = () => {
  const [error, setError] = useState('');
  const [fuelWeb3, setFuelWeb3] = useState<typeof window.fuel>(window.fuel);
  useEffect(() => {
    const timeout = setTimeout(() => {
      if (window.fuel) {
        setFuelWeb3(window.fuel);
      } else {
        setError('FuelWeb3 not detected on the window!');
      }
    }, 500);
    return () => clearTimeout(timeout);
  }, []);

  return [fuelWeb3, error] as const;
};
