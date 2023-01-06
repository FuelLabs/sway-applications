/* eslint-disable @typescript-eslint/no-unused-expressions */
import { yupResolver } from '@hookform/resolvers/yup';
import { useEffect } from 'react';
import { useForm } from 'react-hook-form';
import * as yup from 'yup';

import type { Maybe } from '~/systems/Core';

export type CreateAuctionFormValues = {
  sellerAddress: string;
  sellAssetAmount: string;
  sellAssetId: string;
  sellNFTTokenId: string;
  sellNFTAssetId: string;
  initialPrice: string;
  reservePrice: string;
  bidAssetAmount: string;
  bidAssetId: string;
  bidNFTTokenId: string;
  bidNFTAssetId: string;
  duration: string;
};

const schema = yup
  .object({
    sellerAddress: yup.string().required('Seller address is required'),
    sellAssetAmount: yup.string(),
    sellAssetId: yup.string(),
  })
  .required();

const DEFAULT_VALUES = {
  sellerAddress: '',
  sellAssetAmount: '',
  sellAssetId: '',
  sellNFTTokenId: '',
  sellNFTAssetId: '',
  initialPrice: '',
  reservePrice: '',
  bidAssetAmount: '',
  bidAssetId: '',
  bidNFTTokenId: '',
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
  }, [opts.defaultValues?.name, opts.defaultValues?.url]);

  return form;
}
