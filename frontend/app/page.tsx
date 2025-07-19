'use client';

import { useEffect, useState } from 'react';
import {
  fetchTodos,
  addTodo,
  updateTodo,
  deleteTodo,
  deleteAllTodos,
} from '@/api';
import type { Todo } from '@/types';

export default function Home() {
  const [todos, setTodos] = useState<Todo[]>([]);
  const [text, setText] = useState('');
  const [search, setSearch] = useState('');
  const [editingId, setEditingId] = useState<string | null>(null);

  useEffect(() => {
    loadTodos();
  }, []);

  const loadTodos = async () => {
    try {
      const data = await fetchTodos();
      setTodos(data);
    } catch (err) {
      console.error('Fetch failed:', err);
    }
  };

  const handleAdd = async () => {
    if (!text.trim()) return;

    try {
      if (editingId) {
        await updateTodo(editingId, text);
        setTodos((prev) =>
          prev.map((t) => (t.id === editingId ? { ...t, title: text } : t))
        );
        setEditingId(null);
      } else {
        const newTodo = await addTodo(text);
        setTodos((prev) => [newTodo, ...prev]);
      }
      setText('');
    } catch (err) {
      console.error('Error adding/updating todo:', err);
    }
  };

  const handleDelete = async (id: string) => {
    try {
      const res = await deleteTodo(id);
      if (res.ok) {
        setTodos((prev) => prev.filter((t) => t.id !== id));
      } else {
        console.error('DELETE failed:', await res.text());
      }
    } catch (err) {
      console.error('Error deleting todo:', err);
    }
  };

  const handleDeleteAll = async () => {
    try {
      const res = await deleteAllTodos();
      if (res.ok) {
        setTodos([]); 
        console.log('All todos deleted');
      } else {
        const msg = await res.text();
        console.error(' Failed to delete all:', msg);
      }
    } catch (err) {
      console.error('Error in deleteAll:', err);
    
  };
  
  };

  const handleEdit = (id: string, title: string) => {
    setEditingId(id);
    setText(title);
  };

  const filtered = todos.filter((t) =>
    t.title.toLowerCase().includes(search.toLowerCase())
  );

  return (
    <div className="main-body">
      <div className="todo-app">
        <h1>My Todo App</h1>

        <div className="input-section">
          <input
            type="text"
            placeholder="Search todos..."
            value={search}
            onChange={(e) => setSearch(e.target.value)}
          />
        </div>

        <div className="input-section">
          <input
            type="text"
            placeholder="Add or edit todo..."
            value={text}
            onChange={(e) => setText(e.target.value)}
          />
          <button onClick={handleAdd}>{editingId ? 'Update' : 'Add'}</button>
          <button onClick={handleDeleteAll} className="delete-all">
           Delete All
          </button>
        </div>

        <div className="todos">
          <ul className="todo-list">
            {filtered.map((todo) => (
              <li key={todo.id}>
                <div className="todo-text">{todo.title}</div>
                <div className="actions">
                  <button onClick={() => handleEdit(todo.id, todo.title)}>
                    Edit
                  </button>
                  <button onClick={() => handleDelete(todo.id)}>
                    Delete
                  </button>
                </div>
              </li>
            ))}
          </ul>

          {filtered.length === 0 && (
            <div className="not-found">No todos found</div>
          )}
        </div>
      </div>
    </div>
  );
}
