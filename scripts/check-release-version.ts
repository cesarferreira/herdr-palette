export function releaseVersionError(
  tag: string,
  packageVersion: string,
  manifestVersion: string,
): string | undefined {
  if (!tag.startsWith("v")) {
    return `release tag ${tag} does not start with "v"`;
  }

  const tagVersion = tag.slice(1);
  if (tagVersion !== packageVersion) {
    return `release tag ${tag} does not match package.json version ${packageVersion}`;
  }

  if (manifestVersion !== packageVersion) {
    return `herdr-plugin.toml version ${manifestVersion} does not match package.json version ${packageVersion}`;
  }

  return undefined;
}

if (import.meta.main) {
  const tag = process.argv[2];
  if (!tag) {
    console.error("usage: bun run scripts/check-release-version.ts <tag>");
    process.exit(1);
  }

  const packageJson = await Bun.file("package.json").json();
  const manifest = await Bun.file("herdr-plugin.toml").text();
  const manifestVersion = manifest.match(/^version = "([^"]+)"$/m)?.[1] ?? "";

  const error = releaseVersionError(tag, packageJson.version, manifestVersion);
  if (error) {
    console.error(error);
    process.exit(1);
  }

  console.log(`release ${tag} matches the declared plugin version`);
}
