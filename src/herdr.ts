import type { SessionTarget } from "./types";

const binary = () => process.env.HERDR_BIN_PATH ?? "herdr";

export async function runHerdr(argv: string[]) {
  const child = Bun.spawn([binary(), ...argv], { stdout: "pipe", stderr: "pipe" });
  const [stdout, stderr, code] = await Promise.all([new Response(child.stdout).text(), new Response(child.stderr).text(), child.exited]);
  return { code, stdout, stderr };
}

/** Herdr reports server errors as JSON on stderr; show its message rather than the raw envelope. */
export function explain(stderr: string, code: number) {
  const trimmed = stderr.trim();
  try { const message = JSON.parse(trimmed)?.error?.message; if (message) return String(message); } catch {}
  return trimmed || `Herdr exited with status ${code}.`;
}

/** Herdr injects the pane, tab, and workspace the palette was summoned from as JSON. */
export function parseLaunchContext(json = process.env.HERDR_PLUGIN_CONTEXT_JSON): SessionTarget | undefined {
  if (!json) return undefined;
  try {
    const context = JSON.parse(json);
    const target = { paneId: context.focused_pane_id, tabId: context.tab_id, workspaceId: context.workspace_id };
    return Object.values(target).every(value => typeof value === "string" && value) ? target : undefined;
  } catch { return undefined; }
}

/**
 * The pane the palette acts on. A popup pane has no caller context of its own, so `--current`
 * resolves to the host pane Herdr still considers focused — the fallback when no launch context exists.
 */
export async function sessionTarget(): Promise<SessionTarget | undefined> {
  const launched = parseLaunchContext();
  if (launched) return launched;
  const { code, stdout } = await runHerdr(["pane", "current", "--current"]);
  if (code !== 0) return undefined;
  try {
    const pane = JSON.parse(stdout)?.result?.pane;
    return pane?.pane_id ? { paneId: pane.pane_id, tabId: pane.tab_id, workspaceId: pane.workspace_id } : undefined;
  } catch { return undefined; }
}

export type TabStep = { tabId: string } | { message: string };

export function stepTab(tabIds: string[], current: string, step: number): TabStep {
  if (tabIds.length < 2) return { message: "This workspace only has one tab." };
  const index = tabIds.indexOf(current);
  if (index < 0) return { message: "Herdr did not report the tab that opened the palette." };
  return { tabId: tabIds[(index + step + tabIds.length) % tabIds.length]! };
}

export async function neighborTab(target: SessionTarget, step: number): Promise<TabStep> {
  const { code, stdout, stderr } = await runHerdr(["tab", "list", "--workspace", target.workspaceId]);
  if (code !== 0) return { message: explain(stderr, code) };
  try {
    const tabs: { tab_id: string }[] = JSON.parse(stdout)?.result?.tabs ?? [];
    return stepTab(tabs.map(tab => tab.tab_id), target.tabId, step);
  } catch { return { message: "Herdr returned an unreadable tab list." } }
}
