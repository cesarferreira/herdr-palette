import { expect, test } from "bun:test";
import { defaultItems } from "../src/catalog";
import { explain, parseLaunchContext, stepTab } from "../src/herdr";

test("reads the pane, tab, and workspace Herdr injected at launch", () => {
  const context = JSON.stringify({ workspace_id: "wG", tab_id: "wG:t1", focused_pane_id: "wG:p1", focused_pane_cwd: "/tmp" });

  expect(parseLaunchContext(context)).toEqual({ paneId: "wG:p1", tabId: "wG:t1", workspaceId: "wG" });
});

test("ignores a launch context that is missing a target", () => {
  expect(parseLaunchContext(JSON.stringify({ workspace_id: "wG", tab_id: "wG:t1" }))).toBeUndefined();
  expect(parseLaunchContext("not json")).toBeUndefined();
  expect(parseLaunchContext(undefined)).toBeUndefined();
});

test("steps between tabs in display order and wraps around", () => {
  const tabs = ["w1:t2", "w1:t5", "w1:t9"];

  expect(stepTab(tabs, "w1:t5", 1)).toEqual({ tabId: "w1:t9" });
  expect(stepTab(tabs, "w1:t9", 1)).toEqual({ tabId: "w1:t2" });
  expect(stepTab(tabs, "w1:t2", -1)).toEqual({ tabId: "w1:t9" });
});

test("says why a tab step cannot happen", () => {
  expect(stepTab(["w1:t2"], "w1:t2", 1)).toEqual({ message: "This workspace only has one tab." });
  expect(stepTab(["w1:t2", "w1:t5"], "w2:t1", 1)).toEqual({ message: "Herdr did not report the tab that opened the palette." });
});

test("surfaces the message from a Herdr error envelope", () => {
  expect(explain('{"error":{"code":"no_neighbor","message":"pane has no neighbor"}}', 1)).toBe("pane has no neighbor");
  expect(explain("plain failure", 1)).toBe("plain failure");
  expect(explain("", 2)).toBe("Herdr exited with status 2.");
});

/**
 * Terminal fonts ship wildly different symbol coverage, and a glyph the font lacks falls back to
 * another font with its own metrics — which misaligns the row. Keep icons inside the set verified
 * present at single-cell width in common terminal fonts (ASCII plus arrows and geometric shapes).
 */
test("draws every icon with a glyph terminal fonts actually carry", () => {
  const verified = new Set(["?", "«", "»", "×", "≡", "←", "→", "↑", "↓", "◇", "◈", "▣", "▤", "▧", "▯", "▭", "◲"]);

  for (const item of defaultItems()) {
    expect([...item.icon]).toHaveLength(1);
    expect(verified).toContain(item.icon);
  }
});

test("runs every catalog entry Herdr's CLI can perform", () => {
  const runnable = defaultItems().filter(item => item.invocation.kind !== "shortcut").map(item => item.id);

  expect(runnable).toContain("close_pane");
  expect(runnable).toContain("previous_tab");
  expect(runnable).toContain("next_tab");
  expect(defaultItems().filter(item => item.invocation.kind === "shortcut").map(item => item.id)).toEqual(["help", "settings", "copy_mode", "detach"]);
});
