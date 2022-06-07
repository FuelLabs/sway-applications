import { FC, ChangeEvent, MouseEvent } from "react";
import { removeTodo, toggleTodo } from "~/services/todo";
import { Todo } from "~/todo-contract-types/TodoContractAbi";

type Props = {
  setTodos: (todos: Todo[]) => void;
  todo: Todo;
  index: number;
};

const TodoItem: FC<Props> = ({ todo, index, setTodos }) => {
  const handleToggle = async () => {
    setTodos(await toggleTodo(index));
  };
  const handleRemove = async () => {
    setTodos(await removeTodo(index));
  };

  return (
    <li className={todo.completed ? "completed" : undefined}>
      <div className="view">
        <input
          className="toggle"
          type="checkbox"
          checked={todo.completed}
          onChange={handleToggle}
        />
        <label>{todo.value}</label>
        <button className="destroy" onClick={handleRemove} />
      </div>
    </li>
  );
};

export default TodoItem;
