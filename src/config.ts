import { readFileSync } from "node:fs";
import { defaultItems } from "./catalog";
import type { PaletteItem } from "./types";

const expand = (value: string, prefix: string) => value.split("+").map(part => part === "prefix" ? prefix : part).join("+");

const TOML_ESCAPES: Record<string, string> = { "\\": "\\", '"': '"', t: "\t", n: "\n", r: "\r", b: "\b", f: "\f" };

/** Decode the TOML basic-string escapes we care about so `"prefix+\\"` renders as a single backslash. */
export function unescapeToml(value: string): string {
  return value.replace(/\\(.)/g, (match, char: string) => TOML_ESCAPES[char] ?? match);
}

/** Read bare `name = "binding"` assignments so catalog ids pick up the user's remaps. */
export function parseKeyRemaps(source: string): Map<string, string> {
  const remaps = new Map<string, string>();
  for (const match of source.matchAll(/^([a-z_]+)\s*=\s*"([^"]*)"/gm)) remaps.set(match[1]!, unescapeToml(match[2]!));
  return remaps;
}

export function loadPaletteItems(path = process.env.HERDR_CONFIG_PATH ?? `${process.env.HOME}/.config/herdr/config.toml`): PaletteItem[] {
  const items = defaultItems();
  let source = "";
  try { source = readFileSync(path, "utf8"); } catch {}
  const prefix = unescapeToml(/^prefix\s*=\s*"([^"]+)"/m.exec(source)?.[1] ?? "ctrl+b");
  const remaps = parseKeyRemaps(source);
  return items.map(item => {
    const remapped = remaps.get(item.id);
    const shortcuts = remapped !== undefined
      ? (remapped ? [expand(remapped, prefix)] : [])
      : item.shortcuts.map(key => expand(key, prefix));
    return { ...item, shortcuts };
  });
}
