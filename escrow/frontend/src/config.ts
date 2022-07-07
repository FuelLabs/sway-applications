import { NativeAssetId, toBigInt } from 'fuels';
import path from 'path';

export const FUEL_PROVIDER_URL = 'http://localhost:4000/graphql';

export const FDAI_ID = '0x0101010101010101010101010101010101010101010101010101010101010101';
export const FUSDC_ID = '0x0202020202020202020202020202020202020202020202020202020202020202';
export const FUNI_ID = '0x0303030303030303030303030303030303030303030303030303030303030303';
export const ASSETS = [NativeAssetId, FDAI_ID, FUSDC_ID, FUNI_ID];

export const FETH = {
  name: 'FEther',
  symbol: 'FETH',
  assetId: NativeAssetId,
};

export const FDAI = {
  name: 'FDAI',
  symbol: 'FDAI',
  assetId: FDAI_ID,
};

export const FUSDC = {
  name: 'FUSDc',
  symbol: 'FUSDC',
  assetId: FUSDC_ID,
};

export const FUNI = {
  name: 'FUNI',
  symbol: 'FUNI',
  assetId: FUNI_ID,
};

export const CoinsMetadata = [FETH, FDAI, FUSDC, FUNI];

export const DECIMAL_PLACES = 15;
export const DECIMAL_PRECISION = toBigInt(1e18);

export const ESCROW_ID = process.env.ESCROW_ID!;
