import { describe, it, expect } from "vitest";

describe("Vitest Setup", () => {
  it("should be able to run a simple test", () => {
    expect(true).toBe(true);
  });

  it("should have access to Vitest matchers", () => {
    expect(1 + 1).toBe(2);
    expect("hello").toMatch(/hello/);
    expect([1, 2, 3]).toContain(2);
  });
});
