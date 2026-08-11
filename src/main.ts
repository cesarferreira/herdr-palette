import { Box, Input, Text, createCliRenderer } from "@opentui/core";
import { loadPaletteItems } from "./config";

const theme = { background: "#29284f", panel: "#3c3b68", text: "#e9e8ff", muted: "#a7a4df", accent: "#ffe11a" };
const renderer = await createCliRenderer({ exitOnCtrlC: true });
const items = loadPaletteItems();
const rows = items.slice(0, 14).map((item, index) => Box(
  { flexDirection: "row", width: "100%", paddingLeft: 2, paddingRight: 2, backgroundColor: index === 0 ? theme.panel : theme.background },
  Text({ content: index === 0 ? "┃" : "  ", fg: index === 0 ? theme.accent : theme.background }),
  Text({ content: `${item.icon}  ${item.title}`, fg: index === 0 ? theme.text : theme.muted, flexGrow: 1 }),
  Text({ content: item.shortcuts.join(" / "), fg: index === 0 ? theme.accent : theme.muted }),
));
const search = Input({ placeholder: "Search commands", flexGrow: 1, backgroundColor: theme.background, focusedBackgroundColor: theme.background, textColor: theme.text, cursorColor: theme.accent });
renderer.root.add(Box({ flexDirection: "column", backgroundColor: theme.background, padding: 2, gap: 1 },
  Box({ flexDirection: "row" }, Text({ content: "Commands", fg: theme.text, attributes: 1, flexGrow: 1 }), Text({ content: "esc", fg: theme.muted })),
  Box({ flexDirection: "row", backgroundColor: theme.background }, Text({ content: "┃", fg: theme.accent }), search),
  Text({ content: "Panes", fg: theme.accent, attributes: 1 }),
  ...rows,
 Box({ marginTop: 1 }, Text({ content: `enter select   up/down move   ${items.length} commands`, fg: theme.muted }))
));
search.focus();
renderer.keyInput.on("keypress", (key) => { if (key.name === "escape") renderer.destroy(); });
