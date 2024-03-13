import contractIds from "./contract-types/contract-ids.json";
import productionContractIds from "../production-contract/contract-ids.json";

export const NODE_ENV = import.meta.env.NODE_ENV;
const isProd = NODE_ENV === "production";
export const CONTRACT_ID = isProd ? productionContractIds.tictactoeContract : contractIds.tictactoeContract;
export const PROVIDER_URL = isProd ? "https://beta-5.fuel.network/graphql" : "http://localhost:4000/graphql";
