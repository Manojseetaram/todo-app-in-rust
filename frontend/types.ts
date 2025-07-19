export type Todo = {
  id: string; // 👈 now it's just a string!
  title: string;
  completed: boolean;
  created_at: string;
};

export function decodeUUID(base64: string): string {
  const bytes = Uint8Array.from(atob(base64), c => c.charCodeAt(0));
  const hex = [...bytes].map(b => b.toString(16).padStart(2, '0')).join('');
  return [
    hex.slice(0, 8),
    hex.slice(8, 12),
    hex.slice(12, 16),
    hex.slice(16, 20),
    hex.slice(20),
  ].join('-');
}
