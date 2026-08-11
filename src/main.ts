import { BoxRenderable, InputRenderable, InputRenderableEvents, TextRenderable, createCliRenderer } from "@opentui/core";
import { loadPaletteItems } from "./config";
import { execute } from "./execute";
import { viewport } from "./viewport";

const theme = { background: "#29284f", panel: "#3c3b68", text: "#e9e8ff", muted: "#a7a4df", accent: "#ffe11a" };
const renderer = await createCliRenderer({ exitOnCtrlC: true });
const allItems = loadPaletteItems();
let query = "", selected = 0;
let activePanel: BoxRenderable | undefined;

function matches(item: typeof allItems[number]) {
  const haystack = [item.title, item.description, ...item.aliases, ...item.shortcuts].join(" ").toLowerCase();
  return query.toLowerCase().split(/\s+/).every(token => haystack.includes(token));
}

function redraw() {
  activePanel?.destroy();
  const items = allItems.filter(matches); selected = Math.max(0, Math.min(selected, items.length - 1));
  const panel = new BoxRenderable(renderer, { id: "palette", flexDirection: "column", backgroundColor: theme.background, padding: 2, paddingTop: 0, gap: 1 });
  panel.add(new BoxRenderable(renderer, { id: "heading", flexDirection: "row" }));
  panel.getRenderable("heading")!.add(new TextRenderable(renderer, { id: "title", content: "Commands", fg: theme.text, attributes: 1, flexGrow: 1 }));
  panel.getRenderable("heading")!.add(new TextRenderable(renderer, { id: "escape", content: "esc", fg: theme.muted }));
  const input = new InputRenderable(renderer, { id: "search", value: query, placeholder: "Search commands", backgroundColor: theme.background, focusedBackgroundColor: theme.background, textColor: theme.text, cursorColor: theme.accent });
  input.on(InputRenderableEvents.INPUT, (value: string) => { query = value; selected = 0; redraw(); });
  panel.add(input); input.focus();
  const window = viewport(items, selected, Math.max(5, renderer.height - 7), item => item.category);
  let category = "";
  items.slice(window.start, window.end).forEach((item, offset) => {
    const index = window.start + offset;
    if (item.category !== category) { category = item.category; panel.add(new TextRenderable(renderer, { id: `category-${category}`, content: category, fg: theme.accent, attributes: 1 })); }
    const row = new BoxRenderable(renderer, { id: `item-${index}`, flexDirection: "row", width: "100%", paddingLeft: 1, paddingRight: 2, backgroundColor: index === selected ? theme.panel : theme.background });
    row.add(new TextRenderable(renderer, { id: `mark-${index}`, content: index === selected ? "┃" : " ", fg: theme.accent }));
    row.add(new TextRenderable(renderer, { id: `label-${index}`, content: `${item.icon}  ${item.title}`, fg: index === selected ? theme.text : theme.muted, flexGrow: 1 }));
    row.add(new TextRenderable(renderer, { id: `key-${index}`, content: item.shortcuts.join(" / "), fg: index === selected ? theme.accent : theme.muted })); panel.add(row);
  });
  panel.add(new TextRenderable(renderer, { id: "footer", content: `enter select   ↑/↓ move   ${items.length} commands`, fg: theme.muted, marginTop: 1 })); renderer.root.add(panel); activePanel = panel;
}

renderer.keyInput.on("keypress", async key => {
  const items = allItems.filter(matches);
  if (key.name === "escape") return renderer.destroy();
  if (key.name === "up" || (key.ctrl && key.name === "p")) { selected = (selected - 1 + items.length) % Math.max(1, items.length); return redraw(); }
  if (key.name === "down" || (key.ctrl && key.name === "n")) { selected = (selected + 1) % Math.max(1, items.length); return redraw(); }
  if (key.name === "return" && items[selected]) { const result = await execute(items[selected]); if (result.ok) renderer.destroy(); }
});
redraw();
