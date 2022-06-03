import { Todo } from '~/todo-contract-types/TodoContractAbi';

export const isEmpty = (value: string): boolean =>
  value.trim().length === 0 || new TextEncoder().encode(value).reduce((sum, v) => sum + v, 0) === 0;

const toString = (buffer: Uint8Array): string =>
  String.fromCharCode.apply(null, buffer as unknown as number[]);

export const removeEmptyBytes = (todo: Todo): Todo => ({
  ...todo,
  value: toString(new TextEncoder().encode(todo.value.replace('\x00', '')).filter((v) => v !== 0)),
});

export const isEmptyTodo = (todo: Todo): boolean => isEmpty(todo.value);
export const isValidTodo = (todo: Todo): boolean => !isEmpty(todo.value);
export const getCleanTodos = (todos: Todo[]): Todo[] =>
  todos.filter(isValidTodo).map(removeEmptyBytes);
