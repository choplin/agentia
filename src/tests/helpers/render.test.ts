import { describe, it, expect } from "vitest";

describe("Render Helpers", () => {
  it("should export renderWithContext function", async () => {
    const module = await import("./render");
    expect(module.renderWithContext).toBeDefined();
    expect(typeof module.renderWithContext).toBe("function");
  });
});
