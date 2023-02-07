import type { MutationKey, MutationFilters } from '@tanstack/query-core';
import type { ContextOptions } from './types';
interface Options extends ContextOptions {
}
export declare function useIsMutating(filters?: MutationFilters, options?: Options): number;
export declare function useIsMutating(mutationKey?: MutationKey, filters?: Omit<MutationFilters, 'mutationKey'>, options?: Options): number;
export {};
