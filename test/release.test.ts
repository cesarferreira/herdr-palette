import { expect, test } from "bun:test";
import { releaseVersionError } from "../scripts/check-release-version";
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

test("accepts a release tag matching both declared versions", () => {
  expect(releaseVersionError("v0.3.4", "0.3.4", "0.3.4")).toBeUndefined();
});

test("rejects a release tag that disagrees with the declared versions", () => {
  expect(releaseVersionError("0.3.4", "0.3.4", "0.3.4")).toBe(
    'release tag 0.3.4 does not start with "v"',
  );
  expect(releaseVersionError("v0.3.5", "0.3.4", "0.3.4")).toBe(
    "release tag v0.3.5 does not match package.json version 0.3.4",
  );
  expect(releaseVersionError("v0.3.4", "0.3.4", "0.3.3")).toBe(
    "herdr-plugin.toml version 0.3.3 does not match package.json version 0.3.4",
  );
});
