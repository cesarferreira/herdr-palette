import { explain, neighborTab, runHerdr, sessionTarget } from "./herdr";
import type { CommandResult, Invocation, PaletteItem } from "./types";

type Resolution = { argv: string[] } | { message: string };

/** Some Herdr commands take an explicit target, so they need the pane the palette was summoned from. */
async function resolve(invocation: Exclude<Invocation, { kind: "shortcut" }>): Promise<Resolution> {
  if (invocation.kind === "herdr") return { argv: invocation.argv };
  const target = await sessionTarget();
  if (!target) return { message: "Herdr did not report the pane that opened the palette." };
  if (invocation.kind === "close-pane") return { argv: ["pane", "close", target.paneId] };
  const neighbor = await neighborTab(target, invocation.step);
  return "tabId" in neighbor ? { argv: ["tab", "focus", neighbor.tabId] } : neighbor;
}

export async function execute(item: PaletteItem): Promise<CommandResult> {
  if (item.invocation.kind === "shortcut") return { ok: false, message: `Press ${item.shortcuts.join(" / ")} — Herdr only runs this one from the keyboard.` };
  const resolved = await resolve(item.invocation);
  if ("message" in resolved) return { ok: false, message: resolved.message };
  const { code, stderr } = await runHerdr(resolved.argv);
  return code === 0 ? { ok: true, message: "" } : { ok: false, message: explain(stderr, code) };
}
