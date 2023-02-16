import { useEffect, useState } from "react";

const globalWindow = typeof window !== "undefined" ? window : ({} as Window);

export function useFuel() {
    const [error, setError] = useState("");
    const [isLoading, setLoading] = useState(true);
    const [fuel, setFuel] = useState<Window["fuel"]>(globalWindow.fuel);

    useEffect(() => {
        const timeout = setTimeout(() => {
            if (globalWindow.fuel) {
                setFuel(globalWindow.fuel);
            } else {
                setError("fuel not detected on the window!");
            }
            setLoading(false);
        }, 500);
        return () => clearTimeout(timeout);
    }, []);

    return [fuel as NonNullable<Window["fuel"]>, error, isLoading] as const;
}
