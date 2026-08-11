import { expect, test } from "bun:test";
import manifest from "../herdr-plugin.toml" with { type: "toml" };

test("declares Bun as the popup runtime", async () => {
  const packageJson = await Bun.file("package.json").json();

  expect(packageJson.scripts.test).toBe("bun test");
  expect(packageJson.dependencies["@opentui/core"]).toBeDefined();
  expect(manifest.build[0].command).toEqual(["bun", "install", "--production"]);
  expect(manifest.panes[0].command).toEqual(["bun", "run", "src/main.ts"]);
});
