import { atom } from 'jotai';

export const walletIndexAtom = atom<number | null>(null);
export const showBalancesAtom = atom<boolean>(false);
