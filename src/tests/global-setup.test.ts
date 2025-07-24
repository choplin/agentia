import { describe, it, expect, vi } from "vitest";

describe("Global Test Setup", () => {
  it("should have vi.fn available globally", () => {
    // Vitestのグローバル関数が利用可能であることを確認
    const mockFn = vi.fn();
    expect(mockFn).toBeDefined();
    expect(typeof mockFn).toBe("function");
  });

  it("should have test mode flag set", () => {
    // テストモードフラグが設定されていることを確認
    expect(globalThis.__TEST_MODE__).toBe(true);
  });

  it("should have DOM environment available", () => {
    // jsdom環境が有効であることを確認
    expect(document).toBeDefined();
    expect(window).toBeDefined();
    expect(document.createElement).toBeDefined();
  });

  it("should have fetch available", () => {
    // fetchがグローバルに利用可能であることを確認
    expect(globalThis.fetch).toBeDefined();
    expect(typeof globalThis.fetch).toBe("function");
  });

  it("should have console.error mocked", () => {
    // console.errorがモックされていることを確認
    expect(vi.isMockFunction(console.error)).toBe(true);

    // Svelteエラーが無視されることを確認
    console.error("Svelte warning: something");
    console.error("Normal error");

    // モック関数が呼ばれたことを確認
    expect(console.error).toHaveBeenCalled();
  });

  it("should have console.warn mocked", () => {
    // console.warnがモックされていることを確認
    expect(vi.isMockFunction(console.warn)).toBe(true);

    // 特定の警告が無視されることを確認
    console.warn("This is deprecated");
    console.warn("Normal warning");

    expect(console.warn).toHaveBeenCalled();
  });

  it("should have requestAnimationFrame available", () => {
    // requestAnimationFrameが利用可能であることを確認
    expect(globalThis.requestAnimationFrame).toBeDefined();
    expect(typeof globalThis.requestAnimationFrame).toBe("function");

    expect(globalThis.cancelAnimationFrame).toBeDefined();
    expect(typeof globalThis.cancelAnimationFrame).toBe("function");
  });

  it("should have custom expect matchers available", () => {
    // カスタムマッチャーが利用可能であることを確認
    const div = document.createElement("div");
    expect(div).toBeValidElement();

    // 無効な要素の場合
    expect(() => {
      expect("not an element").toBeValidElement();
    }).toThrow();
  });
});
