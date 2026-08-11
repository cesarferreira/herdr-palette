import { expect, test } from "bun:test";
import { viewport } from "../src/viewport";

test("keeps the selected row inside a fixed visible window", () => {
  expect(viewport(24, 14, 8)).toEqual({ start: 7, end: 15 });
});

test("counts category headers when keeping a selected row visible", () => {
  const items = ["Panes", "Panes", "Tabs", "Tabs", "Tabs"];
  expect(viewport(items, 4, 4, value => value)).toEqual({ start: 2, end: 5 });
});
