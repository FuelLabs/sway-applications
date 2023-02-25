import { useEffect, useState } from 'react';

export function useFuel() {
  const [fuel, setFuel] = useState<Window['fuel']>();

  useEffect(() => {
    const onFuelLoaded = () => {
      setFuel(window.fuel);
    };

    if (window.fuel) {
      onFuelLoaded();
    }

    document.addEventListener('FuelLoaded', onFuelLoaded);

    // On unmount remove the event listener
    return () => {
      document.removeEventListener('FuelLoaded', onFuelLoaded);
    };
  }, []);

  return fuel;
}
