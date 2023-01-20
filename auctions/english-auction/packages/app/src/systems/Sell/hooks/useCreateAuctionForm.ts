/* eslint-disable @typescript-eslint/no-explicit-any */
/* eslint-disable @typescript-eslint/no-unused-expressions */
import { yupResolver } from '@hookform/resolvers/yup';
import { bn, DECIMAL_UNITS, isBech32 } from 'fuels';
import { useEffect } from 'react';
import { useForm } from 'react-hook-form';
import * as yup from 'yup';

import type { Maybe } from '~/systems/Core';

export type CreateAuctionFormValues = {
  sellerAddress: string;
  isSellAssetNft: boolean;
  sellAssetAmount: string;
  sellAssetId: string;
  sellNFTTokenId: string;
  sellNFTAssetId: string;
  initialPrice: string;
  hasReservePrice: boolean;
  reservePrice: string;
  isBidAssetNft: boolean;
  bidAssetId: string;
  bidNFTAssetId: string;
  duration: string;
};

function isValidIdentity(identity: any) {
  try {
    return isBech32(identity);
  } catch (e) {
    return false;
  }
}

function isValidDuration(duration: any) {
  try {
    return !bn(duration).isZero();
  } catch (e) {
    return false;
  }
}

function isReservePriceValid(reservePrice: any, testContext: yup.TestContext) {
  try {
    const bnReservePrice = bn.parseUnits(reservePrice, DECIMAL_UNITS);
    const bnInitialPrice = bn.parseUnits(testContext.parent.initialPrice as string, DECIMAL_UNITS);
    return bnReservePrice.gt(bnInitialPrice);
  } catch (e) {
    return false;
  }
}

// TODO control the boolean values from the dropdown container form thing
// TODO control the boolean value from has reserve price if the value is true require reserve price
// TODO add test/when to check if the reserve price is set
// if it is check that the initial price is less than (or equal?) to the reserve price
const schema = yup
  .object({
    sellerAddress: yup
      .string()
      .test('is-seller-valid', 'Seller is not a valid bech32 address', isValidIdentity)
      .required('Seller address is required'),
    isSellAssetNft: yup.boolean(),
    sellAssetAmount: yup.string().when('isSellAssetNft', {
      is: true,
      then: yup.string().notRequired(),
      otherwise: yup.string().required(),
    }),
    sellAssetId: yup.string().when('isSellAssetNft', {
      is: true,
      then: yup.string().notRequired(),
      otherwise: yup.string().required(),
    }),
    sellNFTTokenId: yup.string().when('isSellAssetNft', {
      is: true,
      then: yup.string().required(),
      otherwise: yup.string().notRequired(),
    }),
    sellNFTAssetId: yup.string().when('isSellAssetNft', {
      is: true,
      then: yup.string().required(),
      otherwise: yup.string().notRequired(),
    }),
    initialPrice: yup.string().when('isBidAssetNft', {
      is: true,
      then: yup.string().notRequired(),
      otherwise: yup.string().required(),
    }),
    hasReservePrice: yup.boolean(),
    reservePrice: yup.string().when('hasReservePrice', {
      is: true,
      then: yup
        .string()
        .test(
          'is-reserve-price-valid',
          'Reserve price must be greater than the initial price',
          isReservePriceValid
        )
        .required(),
      otherwise: yup.string().notRequired(),
    }),
    isBidAssetNft: yup.boolean(),
    bidAssetId: yup.string().when('isBidAssetNft', {
      is: true,
      then: yup.string().notRequired(),
      otherwise: yup.string().required(),
    }),
    bidNFTAssetId: yup.string().when('isBidAssetNft', {
      is: true,
      then: yup.string().required(),
      otherwise: yup.string().notRequired(),
    }),
    duration: yup
      .string()
      .test('is-duration-valid', 'Duration cannot be 0', isValidDuration)
      .required(),
  })
  .required();

const DEFAULT_VALUES = {
  sellerAddress: '',
  isSellAssetNft: false,
  sellAssetAmount: '',
  sellAssetId: '',
  sellNFTTokenId: '',
  sellNFTAssetId: '',
  initialPrice: '',
  hasReservePrice: false,
  reservePrice: '',
  isBidAssetNft: false,
  bidAssetId: '',
  bidNFTAssetId: '',
  duration: '',
};

export type UseCreateAuctionFormReturn = ReturnType<typeof useCreateAuctionForm>;

export type UseCreateAuctionOpts = {
  defaultValues?: Maybe<CreateAuctionFormValues>;
};

export function useCreateAuctionForm(opts: UseCreateAuctionOpts = {}) {
  const form = useForm<CreateAuctionFormValues>({
    resolver: yupResolver(schema),
    reValidateMode: 'onChange',
    mode: 'onChange',
    defaultValues: opts.defaultValues || DEFAULT_VALUES,
  });

  useEffect(() => {
    opts.defaultValues && form.reset(opts.defaultValues);
  }, [opts.defaultValues?.sellerAddress, opts.defaultValues?.sellAssetAmount]);

  return form;
}
