import productionContractIds from '../production-contract/contract-ids.json';

import contractIds from './contract-types/contract-ids.json';

export const IS_PROD = process.env.NODE_ENV === "production";

export const CONTRACT_ID = IS_PROD
  ? productionContractIds.tictactoeContract
  : contractIds.tictactoeContract;
export const PROVIDER_URL = IS_PROD
  ? 'https://beta-5.fuel.network/graphql'
  : 'http://localhost:4000/graphql';
