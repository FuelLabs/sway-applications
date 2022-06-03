import { Wallet } from 'fuels';
import { TodoContractAbi__factory as Factory } from '~/todo-contract-types';

const wallet = new Wallet(process.env.PRIVATE_KEY as string, process.env.FUEL_PROVIDER_URL);

export const contractInstance = Factory.connect(process.env.CONTRACT_ID as string, wallet);
