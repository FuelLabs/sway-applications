import { useEffect, useState } from 'react';

export const useFuelWeb3 = () => {
  const windowLocal = (window as any);
  const [error, setError] = useState('');
  const [fuelWeb3, setFuelWeb3] = useState<any>(windowLocal.FuelWeb3);

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
}
