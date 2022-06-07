import { useState, useEffect, FC, ChangeEvent, KeyboardEvent } from "react";
import TodoList from "~/components/TodoList";
import { Todo } from "~/todo-contract-types/TodoContractAbi";
import { getTodos, addTodo } from "~/services/todo";

const MAX_TODOS = 5;

const App: FC = () => {
  const [todo, setTodo] = useState<string>("");
  const [todos, setTodos] = useState<Todo[]>([]);
  useEffect(() => {
    getTodos().then(setTodos);
  }, [setTodos]);

  const handleChange = (event: ChangeEvent<HTMLInputElement>) => {
    let input = event.target as HTMLInputElement;
    setTodo(input.value);
  };
  const handleNewTodoKeyDown = async (
    event: KeyboardEvent<HTMLInputElement>
  ) => {
    if (event.code !== "Enter") {
      return;
    }

    event.preventDefault();
    let input = event.target as HTMLInputElement;
    setTodos(await addTodo(todos.length, input.value.trim()));
    setTodo("");
  };

  const HAS_MAX = todos.length >= MAX_TODOS;
  return (
    <div>
      <header className="header">
        <h1>todos</h1>
        <input
          autoFocus
          disabled={HAS_MAX}
          maxLength={20}
          className="new-todo"
          value={todo}
          onChange={handleChange}
          onKeyDown={handleNewTodoKeyDown}
          placeholder={
            HAS_MAX ? "Too many Todos - finish some!" : "What needs to be done?"
          }
        />
      </header>
      <TodoList setTodos={setTodos} items={todos} />
    </div>
  );
};

export default App;
