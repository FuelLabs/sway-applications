import contractIds from "./contract-types/contract-ids.json";

export const CONTRACT_ID = contractIds.tictactoeContract;
export const NODE_ENV = import.meta.env.NODE_ENV;
export const PROVIDER_URL = NODE_ENV === "production" ? "https://beta-5.fuel.network/graphql" : "http://localhost:4000/graphql";
