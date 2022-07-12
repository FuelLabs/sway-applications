import { FC } from "react";
import { Todo } from "~/todo-contract-types/TodoContractAbi";
import TodoItem from "./TodoItem";

type Props = {
  setTodos: (todos: Todo[]) => void;
  items: Todo[];
};

const TodoList: FC<Props> = ({ setTodos, items }) =>
  items.length ? (
    <section className="main">
      <ul className="todo-list">
        {items.map((todo, index) => (
          <TodoItem key={index} index={index} todo={todo} setTodos={setTodos} />
        ))}
      </ul>
    </section>
  ) : null;

export default TodoList;
