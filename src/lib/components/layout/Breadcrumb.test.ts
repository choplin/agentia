import { describe, it, expect } from "vitest";
import { render, screen } from "@testing-library/svelte";
import Breadcrumb from "./Breadcrumb.svelte";

describe("Breadcrumb", () => {
  it("should render all breadcrumb items", () => {
    const items = [{ label: "Home" }, { label: "Projects" }, { label: "My Project" }];

    render(Breadcrumb, { props: { items } });

    expect(screen.getByText("Home")).toBeInTheDocument();
    expect(screen.getByText("Projects")).toBeInTheDocument();
    expect(screen.getByText("My Project")).toBeInTheDocument();
  });

  it("should render links when href is provided", () => {
    const items = [
      { label: "Home", href: "/" },
      { label: "Projects", href: "/projects" },
      { label: "Current" }, // No href - should not be a link
    ];

    render(Breadcrumb, { props: { items } });

    const homeLink = screen.getByRole("link", { name: "Home" });
    const projectsLink = screen.getByRole("link", { name: "Projects" });

    expect(homeLink).toHaveAttribute("href", "/");
    expect(projectsLink).toHaveAttribute("href", "/projects");

    // Last item without href should not be a link
    expect(screen.queryByRole("link", { name: "Current" })).not.toBeInTheDocument();
    expect(screen.getByText("Current")).toBeInTheDocument();
  });

  it("should handle empty items array", () => {
    render(Breadcrumb, { props: { items: [] } });

    const nav = screen.getByRole("navigation");
    expect(nav).toBeInTheDocument();
  });
});
