import { useQuery } from "react-query";
import { useFuelWeb3 } from "./useFuelWeb3";

export const useLatestBlockHeight = () => {

    const [fuelWeb3] = useFuelWeb3();

    if (!fuelWeb3) {
        throw Error(`ERROR: fuel web3 is: ${fuelWeb3}`);
    }

    const { data: latestBlockHeight } = useQuery(
        ['latestBlockHeight'],
        async () => {
            return await fuelWeb3.getProvider().getBlockNumber();
        }
    );    

    return latestBlockHeight;
};