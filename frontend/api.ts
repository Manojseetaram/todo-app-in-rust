import type { Todo } from './types';
import { decodeUUID } from './types';

const API_URL = 'http://127.0.0.1:8080';



export async function fetchTodos(): Promise<Todo[]> {
  const res = await fetch('http://127.0.0.1:8080/todos');
  const data = await res.json();

  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  return data.map((todo: any) => ({
    ...todo,
    id: decodeUUID(todo.id.$binary.base64), // <- ID is now a plain string
  }));
}


export async function addTodo(title: string) {
  const res = await fetch(`${API_URL}/todos`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ title, completed: false }),
  });

  if (!res.ok) throw new Error("Failed to add todo");
  const data = await res.json();
  return {
    ...data,
    id: decodeUUID(data.id.$binary.base64),
  };
}

export async function updateTodo(id: string, title: string): Promise<void> {
  const res = await fetch(`${API_URL}/todos/${id}`, {
    method: 'PUT',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ title }),
  });

  if (!res.ok) throw new Error("Failed to update todo");
}

export async function deleteTodo(id: string): Promise<Response> {
  return await fetch(`${API_URL}/todos/${id}`, {
    method: 'DELETE',
  });
}

export async function deleteAllTodos(): Promise<Response> {
  return await fetch(`${API_URL}/todos`, {
    method: 'DELETE',
  });
}
