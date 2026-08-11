export function viewport(length: number, selected: number, capacity: number): { start: number; end: number };
export function viewport<T>(items: T[], selected: number, capacity: number, category: (item: T) => string): { start: number; end: number };
export function viewport<T>(source: number | T[], selected: number, capacity: number, category?: (item: T) => string) {
  if (typeof source === "number") {
    const start = Math.max(0, Math.min(selected - capacity + 1, source - capacity));
    return { start, end: Math.min(source, start + capacity) };
  }
  const cost = (start: number, end: number) => {
    let rows = 0, previous = "";
    for (let index = start; index < end; index++) {
      const current = category!(source[index]);
      if (index === start || current !== previous) rows++;
      rows++; previous = current;
    }
    return rows;
  };
  let start = selected;
  while (start > 0 && cost(start - 1, selected + 1) <= capacity) start--;
  let end = selected + 1;
  while (end < source.length && cost(start, end + 1) <= capacity) end++;
  return { start, end };
}
