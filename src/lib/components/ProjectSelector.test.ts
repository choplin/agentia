import { describe, it, expect, vi } from "vitest";
import { render, screen, fireEvent } from "@testing-library/svelte";
import ProjectSelector from "./ProjectSelector.svelte";
import type { Project } from "$lib/claude";

const mockProjects: Project[] = [
  {
    id: "proj-1",
    name: "Project A",
    path: "/path/to/a",
    createdAt: "2024-01-01",
    updatedAt: "2024-01-01",
    settings: null,
  },
  {
    id: "proj-2",
    name: "Project B",
    path: "/path/to/b",
    createdAt: "2024-01-02",
    updatedAt: "2024-01-02",
    settings: null,
  },
  {
    id: "proj-3",
    name: "Project C",
    path: "/path/to/c",
    createdAt: "2024-01-03",
    updatedAt: "2024-01-03",
    settings: null,
  },
];

describe("ProjectSelector", () => {
  it("should display current project name or placeholder", () => {
    render(ProjectSelector, {
      props: {
        projects: mockProjects,
        currentProject: mockProjects[0],
        onProjectSelect: vi.fn(),
        onProjectCreate: vi.fn(),
      },
    });

    expect(screen.getByText("Project A")).toBeInTheDocument();
    expect(screen.getByText("/path/to/a")).toBeInTheDocument();
  });

  it("should display 'Select Project' when no current project", () => {
    render(ProjectSelector, {
      props: {
        projects: mockProjects,
        currentProject: null,
        onProjectSelect: vi.fn(),
        onProjectCreate: vi.fn(),
      },
    });

    expect(screen.getByText("Select Project")).toBeInTheDocument();
  });

  it("should open dropdown when clicked", async () => {
    render(ProjectSelector, {
      props: {
        projects: mockProjects,
        currentProject: null,
        onProjectSelect: vi.fn(),
        onProjectCreate: vi.fn(),
      },
    });

    const button = screen.getByText("Select Project").closest("button");
    await fireEvent.click(button!);

    // Should show all projects
    expect(screen.getByText("Project A")).toBeInTheDocument();
    expect(screen.getByText("Project B")).toBeInTheDocument();
    expect(screen.getByText("Project C")).toBeInTheDocument();
    expect(screen.getByText("Create New Project")).toBeInTheDocument();
  });

  it("should call onProjectSelect when project is clicked", async () => {
    const handleSelect = vi.fn();

    render(ProjectSelector, {
      props: {
        projects: mockProjects,
        currentProject: null,
        onProjectSelect: handleSelect,
        onProjectCreate: vi.fn(),
      },
    });

    // Open dropdown
    const button = screen.getByText("Select Project").closest("button");
    await fireEvent.click(button!);

    // Click on Project B
    await fireEvent.click(screen.getByText("Project B"));

    expect(handleSelect).toHaveBeenCalledWith(mockProjects[1]);
  });

  it("should call onProjectCreate when create button is clicked", async () => {
    const handleCreate = vi.fn();

    render(ProjectSelector, {
      props: {
        projects: mockProjects,
        currentProject: null,
        onProjectSelect: vi.fn(),
        onProjectCreate: handleCreate,
      },
    });

    // Open dropdown
    const button = screen.getByText("Select Project").closest("button");
    await fireEvent.click(button!);

    // Click create button
    await fireEvent.click(screen.getByText("Create New Project"));

    expect(handleCreate).toHaveBeenCalled();
  });

  it("should close dropdown when project is selected", async () => {
    render(ProjectSelector, {
      props: {
        projects: mockProjects,
        currentProject: null,
        onProjectSelect: vi.fn(),
        onProjectCreate: vi.fn(),
      },
    });

    // Open dropdown
    const button = screen.getByText("Select Project").closest("button");
    await fireEvent.click(button!);

    // Verify dropdown is open
    expect(screen.getByText("Project A")).toBeInTheDocument();

    // Click on a project
    await fireEvent.click(screen.getByText("Project A"));

    // Dropdown should close - only the selected project name should be visible
    expect(screen.queryByText("Project B")).not.toBeInTheDocument();
  });

  it("should close dropdown when clicking outside", async () => {
    render(ProjectSelector, {
      props: {
        projects: mockProjects,
        currentProject: null,
        onProjectSelect: vi.fn(),
        onProjectCreate: vi.fn(),
      },
    });

    // Open dropdown
    const button = screen.getByText("Select Project").closest("button");
    await fireEvent.click(button!);

    // Click the invisible overlay
    const overlay = screen.getByLabelText("Close project selector");
    await fireEvent.click(overlay);

    // Dropdown should close
    expect(screen.queryByText("Project B")).not.toBeInTheDocument();
  });
});
