export type Category = "Workspace" | "Tabs" | "Panes" | "Herdr" | "Custom";
export type Invocation = { kind: "herdr"; argv: string[] } | { kind: "close-pane" } | { kind: "focus-tab"; step: -1 | 1 } | { kind: "shortcut" };
export interface SessionTarget { paneId: string; tabId: string; workspaceId: string }
export interface PaletteItem { id: string; title: string; category: Category; description: string; icon: string; aliases: string[]; shortcuts: string[]; invocation: Invocation; }
export interface CommandResult { ok: boolean; message: string }
