import contractIds from './contract-types/contract-ids.json';
import productionContractIds from '../production-contract/contract-ids.json';

export const CONTRACT_ID = import.meta.env.PROD
  ? productionContractIds.tictactoeContract
  : contractIds.tictactoeContract;
export const PROVIDER_URL = import.meta.env.PROD
  ? 'https://beta-5.fuel.network/graphql'
  : 'http://localhost:4000/graphql';
