import { defaultItems } from "./catalog"; import type { PaletteItem } from "./types";
const expand=(value:string,prefix:string)=>value.split("+").map(x=>x==="prefix"?prefix:x).join("+");
export function loadPaletteItems(path=process.env.HERDR_CONFIG_PATH ?? `${process.env.HOME}/.config/herdr/config.toml`):PaletteItem[]{ const items=defaultItems(); let source=""; try{source=readFileSync(path,"utf8")}catch{} const prefix=/^prefix\s*=\s*"([^"]+)"/m.exec(source)?.[1]??"ctrl+b"; return items.map(item=>({...item,shortcuts:item.shortcuts.map(key=>expand(key,prefix))})); }
import { readFileSync } from "node:fs";
