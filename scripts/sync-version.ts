export function syncManifestVersion(manifest: string, version: string): string {
  if (!/^version = "[^"]+"$/m.test(manifest)) {
    throw new Error("herdr-plugin.toml has no version field");
  }

  return manifest.replace(/^version = "[^"]+"$/m, `version = "${version}"`);
}

if (import.meta.main) {
  const packageJson = await Bun.file("package.json").json();
  const manifestFile = Bun.file("herdr-plugin.toml");
  const manifest = await manifestFile.text();

  await Bun.write(manifestFile, syncManifestVersion(manifest, packageJson.version));
}
