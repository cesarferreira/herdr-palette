import { expect, test } from "bun:test";
import { syncManifestVersion } from "../scripts/sync-version";

test("syncs the plugin manifest to the package release version", () => {
  const manifest = `id = "cesarferreira.herdr-palette"
name = "Herdr Palette"
version = "0.1.0"
min_herdr_version = "0.7.0"
`;

  expect(syncManifestVersion(manifest, "0.3.0")).toBe(
    `id = "cesarferreira.herdr-palette"
name = "Herdr Palette"
version = "0.3.0"
min_herdr_version = "0.7.0"
`,
  );
});
