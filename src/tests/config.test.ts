import { describe, it, expect } from "vitest";
import { existsSync } from "fs";
import { resolve } from "path";
import type { UserConfig } from "vitest/config";

describe("vitest.config.ts", () => {
  it("should exist in the project root", () => {
    const configPath = resolve(process.cwd(), "vitest.config.ts");
    expect(existsSync(configPath)).toBe(true);
  });

  it("should be importable and have valid configuration", async () => {
    const configModule = await import("../../vitest.config");
    const config = configModule.default as UserConfig;

    expect(config).toBeDefined();
    expect(config.test).toBeDefined();
    expect(config.test!.environment).toBe("happy-dom");
    expect(config.test!.globals).toBe(true);
    expect(config.test!.setupFiles).toContain("./src/tests/setup.ts");
  });

  it("should have coverage configuration", async () => {
    const configModule = await import("../../vitest.config");
    const config = configModule.default as UserConfig;

    expect(config.test?.coverage).toBeDefined();
    const coverage = config.test!.coverage as any;
    expect(coverage.provider).toBe("v8");
    expect(coverage.reporter).toEqual(["text", "html", "lcov"]);
    expect(coverage.exclude).toContain("node_modules/**");
    expect(coverage.exclude).toContain("**/*.config.*");
    expect(coverage.exclude).toContain("**/*.d.ts");
  });

  it("should have proper coverage thresholds", async () => {
    const configModule = await import("../../vitest.config");
    const config = configModule.default as UserConfig;

    const coverage = config.test!.coverage as any;
    const thresholds = coverage.thresholds;
    expect(thresholds).toBeDefined();
    expect(thresholds.lines).toBe(80);
    expect(thresholds.functions).toBe(80);
    expect(thresholds.branches).toBe(80);
    expect(thresholds.statements).toBe(80);
  });
});
