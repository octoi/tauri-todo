import { createState, Downgraded } from '@hookstate/core';

interface Todo {
  id: number;
  title: string;
  resolved: boolean;
}

export const todoStore = createState<Todo[]>([]);

export const getTodos = () => {
  return todoStore.attach(Downgraded).get();
};

export const addTodo = (title: string) => {
  let items = getTodos();

  let todo: Todo = {
    id: items.length,
    title,
    resolved: false,
  };

  items.unshift(todo);
  todoStore.set(items);
};

export const updateTodo = (updatedTodo: Todo) => {
  let items = getTodos();

  items.filter((todo) => (todo.id == updatedTodo.id ? updatedTodo : todo));
  todoStore.set(items);
};

export const deleteTodo = (id: number) => {
  let items = getTodos();
  items.filter((todo) => todo.id !== id);
  todoStore.set(items);
};
