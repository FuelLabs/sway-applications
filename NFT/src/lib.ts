import { Account, BN } from "fuels";
import productionContractId from "../production-contract/contract-ids.json";
import contractId from "@/contract-types/contract-ids.json";

type DappEnvironment = "local" | "testnet";

export const IS_PROD = process.env.NODE_ENV === "production";

export const CURRENT_ENVIRONMENT: DappEnvironment = IS_PROD
  ? "testnet"
  : "local";

export const NODE_URL =
  CURRENT_ENVIRONMENT === "local"
    ? `http://127.0.0.1:${process.env.NEXT_PUBLIC_FUEL_NODE_PORT || 4000}/v1/graphql`
    : "https://testnet.fuel.network/v1/graphql";

/**
 * Enable the Fuel dev connector.
 * @see {@link https://docs.fuel.network/docs/wallet/dev/getting-started/#using-default-connectors}
 */
export const ENABLE_FUEL_DEV_CONNECTOR =
  process.env.NEXT_PUBLIC_ENABLE_FUEL_DEV_CONNECTOR === "true";

export interface AppWallet {
  wallet?: Account;
  walletBalance?: BN;
  refreshWalletBalance?: () => Promise<void>;
}

export const TESTNET_FAUCET_LINK = "https://faucet-testnet.fuel.network/";

export const GATEWAY_URL = process.env.NEXT_PUBLIC_GATEWAY_URL
  ? process.env.NEXT_PUBLIC_GATEWAY_URL
  : "https://gateway.pinata.cloud";

export const CONTRACT_ID = IS_PROD
  ? productionContractId["nftContract"]
  : contractId["nftContract"];
