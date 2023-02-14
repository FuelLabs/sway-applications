import { toast } from "@fuel-ui/react";
import { useQuery } from "@tanstack/react-query";
import useFuel from "./useFuel";

const useProvider = () => {
    const [fuel] = useFuel();

    if (!fuel) toast.error("Error fuelWeb3 instance is not defined");

    const {
        data: provider,
        isLoading,
        isError,
    } = useQuery(
        ["provider"],
        async () => {
            const isConnected = await fuel.isConnected();
            if (!isConnected) {
                await fuel.connect();
            }
            const fuelProvider = await fuel.getProvider();
            return fuelProvider;
        },
        {
            enabled: !!fuel,
        }
    );

    return { provider, isLoading, isError };
};

export default useProvider;
