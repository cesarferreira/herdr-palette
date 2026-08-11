export function viewport(length: number, selected: number, capacity: number) {
  const start = Math.max(0, Math.min(selected - capacity + 1, length - capacity));
  return { start, end: Math.min(length, start + capacity) };
}
