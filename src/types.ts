export type Category = "Workspace" | "Tabs" | "Panes" | "Herdr" | "Custom";
export type Invocation = { kind: "herdr"; argv: string[] } | { kind: "shortcut" };
export interface PaletteItem { id: string; title: string; category: Category; description: string; icon: string; aliases: string[]; shortcuts: string[]; invocation: Invocation; }
