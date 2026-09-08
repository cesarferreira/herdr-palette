import { createCliRenderer } from "@opentui/core";
import { loadPaletteItems } from "./config";
import { execute } from "./execute";
import { mountPalette } from "./palette";
import { loadTheme } from "./theme";

// Read Herdr's config before drawing so the popup uses the theme Herdr itself is rendering with.
const theme = loadTheme();
const renderer = await createCliRenderer({ exitOnCtrlC: true, backgroundColor: theme.background });
mountPalette(renderer, loadPaletteItems(), { theme, run: execute, close: () => renderer.destroy() });
