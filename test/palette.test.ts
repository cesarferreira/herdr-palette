import { createTestRenderer } from "@opentui/core/testing";
import { expect, test } from "bun:test";
import { mountPalette, theme } from "../src/palette";
import type { CommandResult, PaletteItem } from "../src/types";

const item = (id: string, title: string, invocation: PaletteItem["invocation"]): PaletteItem =>
  ({ id, title, category: "Panes", description: "", icon: "▯", aliases: [], shortcuts: ["ctrl+a+z"], invocation });

const items = [
  item("zoom", "Zoom pane", { kind: "herdr", argv: ["pane", "zoom", "--current"] }),
  item("settings", "Settings", { kind: "shortcut" }),
];

async function palette(result: CommandResult) {
  const harness = await createTestRenderer({ width: 60, height: 14 });
  const ran: string[] = [];
  mountPalette(harness.renderer, items, { run: async selected => { ran.push(selected.id); return result; }, close: () => ran.push("closed") });
  return { ...harness, ran };
}

const hex = (color: { r: number; g: number; b: number }) =>
  "#" + [color.r, color.g, color.b].map(value => Math.round(value <= 1 ? value * 255 : value).toString(16).padStart(2, "0")).join("");

/** Enter runs the command asynchronously, so let its result land before rendering. */
const settle = () => new Promise(resolve => setTimeout(resolve, 0));
const rowsOf = (frame: string) => frame.split("\n").filter((row, index, all) => index < all.length - 1 || row !== "");

test("paints the palette background across the whole popup", async () => {
  const { renderer, mockInput, renderOnce, captureSpans } = await palette({ ok: true, message: "" });

  await mockInput.typeText("settings");
  await renderOnce();

  const frame = captureSpans();
  expect(frame.lines).toHaveLength(renderer.height);
  for (const line of frame.lines.slice(0, -1)) expect(line.spans.map(span => hex(span.bg))).toContain(theme.background);
});

test("pins the footer to the bottom of the popup", async () => {
  const { mockInput, renderOnce, captureCharFrame } = await palette({ ok: true, message: "" });

  await mockInput.typeText("settings");
  await renderOnce();

  const rows = rowsOf(captureCharFrame());
  expect(rows).toHaveLength(14);
  expect(rows.at(-1)).toContain("1 commands");
});

test("sets the footer apart as a full-width bar", async () => {
  const { renderer, mockInput, renderOnce, captureSpans } = await palette({ ok: true, message: "" });

  await mockInput.typeText("settings");
  await renderOnce();

  const footer = captureSpans().lines.at(-1)!;
  expect(footer.spans.map(span => hex(span.bg))).toEqual(footer.spans.map(() => theme.footer));
  expect(footer.spans.reduce((width, span) => width + span.text.length, 0)).toBe(renderer.width);
  expect(footer.spans.filter(span => hex(span.fg) === theme.accent).map(span => span.text)).toEqual(["enter", "↑/↓"]);
});

test("closes the palette once a Herdr command succeeds", async () => {
  const { mockInput, renderOnce, ran } = await palette({ ok: true, message: "" });

  await mockInput.typeText("zoom");
  mockInput.pressEnter();
  await settle();
  await renderOnce();

  expect(ran).toEqual(["zoom", "closed"]);
});

test("reports why a command did not run instead of ignoring enter", async () => {
  const { mockInput, renderOnce, captureCharFrame } = await palette({ ok: false, message: "Press ctrl+a+z — Herdr only runs this one from the keyboard." });

  await mockInput.typeText("settings");
  mockInput.pressEnter();
  await settle();
  await renderOnce();

  const rows = rowsOf(captureCharFrame());
  expect(rows).toHaveLength(14);
  expect(rows.at(-2)).toContain("Press ctrl+a+z — Herdr only runs this one");
  expect(rows.at(-1)).toContain("1 commands");
});

test("stays usable when running a command throws", async () => {
  const { renderer, mockInput, renderOnce, captureCharFrame } = await createTestRenderer({ width: 60, height: 14 });
  mountPalette(renderer, items, { run: async () => { throw new Error("herdr is not on PATH"); }, close: () => {} });

  await mockInput.typeText("zoom");
  mockInput.pressEnter();
  await settle();
  mockInput.pressEnter();
  await settle();
  await renderOnce();

  expect(captureCharFrame()).toContain("herdr is not on PATH");
});

test("explains an empty result set", async () => {
  const { mockInput, renderOnce, captureCharFrame } = await palette({ ok: true, message: "" });

  await mockInput.typeText("nowhere");
  await renderOnce();

  expect(captureCharFrame()).toContain("No commands match your search.");
});
