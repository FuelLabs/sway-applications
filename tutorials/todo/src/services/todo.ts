import { Todo } from '~/todo-contract-types/TodoContractAbi';
import { contractInstance } from '~/services/contract';
import { getCleanTodos } from '~/utils/todo-helpers';

const logFailed = (name: string) => (error: unknown) =>
  console.error(`calling submit on ${name} failed with:`, error);

export const getTodos = async (): Promise<Todo[]> => {
  try {
    let todos = await contractInstance.submit.get_todos();
    return getCleanTodos(todos);
  } catch (error) {
    logFailed('getTodos')(error);
  }

  return [];
};

export const addTodo = async (index: number, value: string): Promise<Todo[]> => {
  try {
    let todos = await contractInstance.submit.add_todo(index, value);
    return getCleanTodos(todos);
  } catch (error) {
    logFailed('addTodo')(error);
  }

  return [];
};

export const toggleTodo = async (index: number): Promise<Todo[]> => {
  try {
    let todos = await contractInstance.submit.toggle_todo(index);
    return getCleanTodos(todos);
  } catch (error) {
    logFailed('toggleTodo')(error);
  }

  return [];
};

export const removeTodo = async (index: number): Promise<Todo[]> => {
  try {
    let todos = await contractInstance.submit.remove_todo(index);
    return getCleanTodos(todos);
  } catch (error) {
    logFailed('removeTodo')(error);
  }

  return [];
};
