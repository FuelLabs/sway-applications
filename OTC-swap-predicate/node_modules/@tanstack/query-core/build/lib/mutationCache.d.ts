import type { MutationObserver } from './mutationObserver';
import type { MutationOptions } from './types';
import type { QueryClient } from './queryClient';
import type { Action, MutationState } from './mutation';
import { Mutation } from './mutation';
import type { MutationFilters } from './utils';
import { Subscribable } from './subscribable';
interface MutationCacheConfig {
    onError?: (error: unknown, variables: unknown, context: unknown, mutation: Mutation<unknown, unknown, unknown>) => Promise<unknown> | unknown;
    onSuccess?: (data: unknown, variables: unknown, context: unknown, mutation: Mutation<unknown, unknown, unknown>) => Promise<unknown> | unknown;
    onMutate?: (variables: unknown, mutation: Mutation<unknown, unknown, unknown, unknown>) => Promise<unknown> | unknown;
}
interface NotifyEventMutationAdded {
    type: 'added';
    mutation: Mutation<any, any, any, any>;
}
interface NotifyEventMutationRemoved {
    type: 'removed';
    mutation: Mutation<any, any, any, any>;
}
interface NotifyEventMutationObserverAdded {
    type: 'observerAdded';
    mutation: Mutation<any, any, any, any>;
    observer: MutationObserver<any, any, any>;
}
interface NotifyEventMutationObserverRemoved {
    type: 'observerRemoved';
    mutation: Mutation<any, any, any, any>;
    observer: MutationObserver<any, any, any>;
}
interface NotifyEventMutationObserverOptionsUpdated {
    type: 'observerOptionsUpdated';
    mutation?: Mutation<any, any, any, any>;
    observer: MutationObserver<any, any, any, any>;
}
interface NotifyEventMutationUpdated {
    type: 'updated';
    mutation: Mutation<any, any, any, any>;
    action: Action<any, any, any, any>;
}
declare type MutationCacheNotifyEvent = NotifyEventMutationAdded | NotifyEventMutationRemoved | NotifyEventMutationObserverAdded | NotifyEventMutationObserverRemoved | NotifyEventMutationObserverOptionsUpdated | NotifyEventMutationUpdated;
declare type MutationCacheListener = (event: MutationCacheNotifyEvent) => void;
export declare class MutationCache extends Subscribable<MutationCacheListener> {
    config: MutationCacheConfig;
    private mutations;
    private mutationId;
    constructor(config?: MutationCacheConfig);
    build<TData, TError, TVariables, TContext>(client: QueryClient, options: MutationOptions<TData, TError, TVariables, TContext>, state?: MutationState<TData, TError, TVariables, TContext>): Mutation<TData, TError, TVariables, TContext>;
    add(mutation: Mutation<any, any, any, any>): void;
    remove(mutation: Mutation<any, any, any, any>): void;
    clear(): void;
    getAll(): Mutation[];
    find<TData = unknown, TError = unknown, TVariables = any, TContext = unknown>(filters: MutationFilters): Mutation<TData, TError, TVariables, TContext> | undefined;
    findAll(filters: MutationFilters): Mutation[];
    notify(event: MutationCacheNotifyEvent): void;
    resumePausedMutations(): Promise<void>;
}
export {};
