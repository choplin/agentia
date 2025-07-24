import { describe, it, expect } from "vitest";
import { render, screen, fireEvent } from "@testing-library/svelte";
import TestComponent from "./TestComponent.svelte";

describe("Svelte Component Rendering in DOM Environment", () => {
  it("should render a Svelte component with props", async () => {
    const { container } = render(TestComponent, {
      props: {
        message: "Hello Svelte!",
        count: 0,
      },
    });

    const component = container.querySelector(".test-component");
    expect(component).toBeTruthy();

    const heading = container.querySelector("h1");
    expect(heading?.textContent).toBe("Hello Svelte!");

    const countText = container.querySelector("p");
    expect(countText?.textContent).toBe("Count: 0");
  });

  it("should update reactive state when button is clicked", async () => {
    const { container } = render(TestComponent, {
      props: {
        message: "Interactive Test",
        count: 0,
      },
    });

    const button = screen.getByRole("button", { name: "Increment" });
    const countText = container.querySelector("p");

    expect(countText?.textContent).toBe("Count: 0");

    // Click the button
    await fireEvent.click(button);

    // Check that count is updated
    expect(countText?.textContent).toBe("Count: 1");

    // Click again
    await fireEvent.click(button);

    // Check that count is updated again
    expect(countText?.textContent).toBe("Count: 2");
  });

  it("should render Svelte 5 component with $props and $bindable", () => {
    const { container } = render(TestComponent, {
      props: {
        message: "Svelte 5 Runes Mode",
      },
    });

    const heading = container.querySelector("h1");
    expect(heading?.textContent).toBe("Svelte 5 Runes Mode");

    // Default count should be 0
    const countText = container.querySelector("p");
    expect(countText?.textContent).toBe("Count: 0");
  });

  it("should handle component lifecycle in DOM environment", () => {
    const { container, unmount } = render(TestComponent, {
      props: {
        message: "Lifecycle Test",
      },
    });

    // Component should be rendered
    const component = container.querySelector(".test-component");
    expect(component).toBeTruthy();

    // Unmount the component
    unmount();

    // Component should be removed from DOM
    const unmountedComponent = container.querySelector(".test-component");
    expect(unmountedComponent).toBeFalsy();
  });

  it("should support DOM queries and assertions", () => {
    const { container } = render(TestComponent, {
      props: {
        message: "DOM Query Test",
      },
    });

    // Query by selector
    const button = container.querySelector("button");
    expect(button).toBeTruthy();
    expect(button?.textContent).toBe("Increment");

    // Query by tag
    const heading = container.querySelector("h1");
    expect(heading).toBeTruthy();
    expect(heading?.tagName).toBe("H1");
    expect(heading?.textContent).toBe("DOM Query Test");
  });

  it("should verify DOM environment capabilities", () => {
    // DOM APIが利用可能なことを確認
    expect(typeof window).toBe("object");
    expect(typeof document).toBe("object");
    expect(typeof document.createElement).toBe("function");
    expect(typeof document.querySelector).toBe("function");

    // happy-dom環境であることを確認
    const testDiv = document.createElement("div");
    testDiv.className = "test-dom-element";
    document.body.appendChild(testDiv);

    const foundDiv = document.querySelector(".test-dom-element");
    expect(foundDiv).toBeTruthy();
    expect(foundDiv?.className).toBe("test-dom-element");

    // クリーンアップ
    document.body.removeChild(testDiv);
  });
});
