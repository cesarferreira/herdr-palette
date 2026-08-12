import { readFileSync } from "node:fs";
import { defaultItems } from "./catalog";
import type { PaletteItem } from "./types";

const expand = (value: string, prefix: string) => value.split("+").map(part => part === "prefix" ? prefix : part).join("+");

/** Read bare `name = "binding"` assignments so catalog ids pick up the user's remaps. */
export function parseKeyRemaps(source: string): Map<string, string> {
  const remaps = new Map<string, string>();
  for (const match of source.matchAll(/^([a-z_]+)\s*=\s*"([^"]*)"/gm)) remaps.set(match[1]!, match[2]!);
  return remaps;
}

export function loadPaletteItems(path = process.env.HERDR_CONFIG_PATH ?? `${process.env.HOME}/.config/herdr/config.toml`): PaletteItem[] {
  const items = defaultItems();
  let source = "";
  try { source = readFileSync(path, "utf8"); } catch {}
  const prefix = /^prefix\s*=\s*"([^"]+)"/m.exec(source)?.[1] ?? "ctrl+b";
  const remaps = parseKeyRemaps(source);
  return items.map(item => {
    const remapped = remaps.get(item.id);
    const shortcuts = remapped !== undefined
      ? (remapped ? [expand(remapped, prefix)] : [])
      : item.shortcuts.map(key => expand(key, prefix));
    return { ...item, shortcuts };
  });
}
