import { expect, test } from "bun:test";
import { viewport } from "../src/viewport";

test("keeps the selected row inside a fixed visible window", () => {
  expect(viewport(24, 14, 8)).toEqual({ start: 7, end: 15 });
});
