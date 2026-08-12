import { createCliRenderer } from "@opentui/core";
import { loadPaletteItems } from "./config";
import { execute } from "./execute";
import { mountPalette, theme } from "./palette";

const renderer = await createCliRenderer({ exitOnCtrlC: true, backgroundColor: theme.background });
mountPalette(renderer, loadPaletteItems(), { run: execute, close: () => renderer.destroy() });
