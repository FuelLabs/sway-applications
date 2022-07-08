import { useContext } from "react";

import { AppContext } from "../context/AppContext";

export const useContract = () => {
    const context = useContext(AppContext);
    return context?.contract;
}