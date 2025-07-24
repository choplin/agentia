import { describe, it, expect } from "vitest";

describe("Vitest Environment Configuration", () => {
  it("should run tests in jsdom environment", () => {
    // jsdom環境でのみ利用可能なグローバルオブジェクトを確認
    expect(typeof window).toBe("object");
    expect(typeof document).toBe("object");
    expect(typeof navigator).toBe("object");
  });

  it("should have DOM manipulation capabilities", () => {
    // DOM要素の作成と操作が可能なことを確認
    const div = document.createElement("div");
    div.textContent = "Hello, jsdom!";
    document.body.appendChild(div);

    const element = document.querySelector("div");
    expect(element).toBeTruthy();
    expect(element?.textContent).toBe("Hello, jsdom!");

    // クリーンアップ
    document.body.removeChild(div);
  });

  it("should support window.localStorage", () => {
    // localStorageが利用可能なことを確認
    localStorage.setItem("test-key", "test-value");
    expect(localStorage.getItem("test-key")).toBe("test-value");

    // クリーンアップ
    localStorage.clear();
  });

  it("should support custom event dispatching", () => {
    // カスタムイベントの発火と受信が可能なことを確認
    let eventFired = false;

    const handleEvent = () => {
      eventFired = true;
    };

    window.addEventListener("custom-event", handleEvent);
    window.dispatchEvent(new CustomEvent("custom-event"));

    expect(eventFired).toBe(true);

    // クリーンアップ
    window.removeEventListener("custom-event", handleEvent);
  });
});
