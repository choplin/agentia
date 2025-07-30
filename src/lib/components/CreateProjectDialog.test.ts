import { describe, it, expect, beforeEach, vi } from "vitest";
import { render, screen, fireEvent, waitFor } from "@testing-library/svelte";
import { mockIPC, clearMocks } from "@tauri-apps/api/mocks";
import CreateProjectDialog from "./CreateProjectDialog.svelte";

// Mock the dialog plugin
vi.mock("@tauri-apps/plugin-dialog", () => ({
  open: vi.fn(),
}));

describe("CreateProjectDialog", () => {
  const mockHandlers = {
    onClose: vi.fn(),
    onProjectCreated: vi.fn(),
  };

  beforeEach(() => {
    clearMocks();
    vi.clearAllMocks();
  });

  it("should render when isOpen is true", () => {
    render(CreateProjectDialog, {
      props: {
        isOpen: true,
        ...mockHandlers,
      },
    });

    expect(screen.getByText("Create New Project")).toBeInTheDocument();
    expect(screen.getByLabelText("Project Name")).toBeInTheDocument();
    expect(screen.getByLabelText("Project Directory")).toBeInTheDocument();
  });

  it("should not render when isOpen is false", () => {
    render(CreateProjectDialog, {
      props: {
        isOpen: false,
        ...mockHandlers,
      },
    });

    expect(screen.queryByText("Create New Project")).not.toBeInTheDocument();
  });

  it("should disable create button when fields are empty", () => {
    render(CreateProjectDialog, {
      props: {
        isOpen: true,
        ...mockHandlers,
      },
    });

    const createButton = screen.getByText("Create Project");
    expect(createButton).toBeDisabled();
  });

  it("should create a project successfully", async () => {
    const newProject = {
      id: 1,
      name: "Test Project",
      path: "/home/user/test-project",
      createdAt: "2024-01-20T00:00:00Z",
      updatedAt: "2024-01-20T00:00:00Z",
    };

    mockIPC((cmd, args) => {
      if (cmd === "create_project") {
        expect(args).toEqual({ name: "Test Project", path: "/home/user/test-project" });
        return newProject;
      }
    });

    render(CreateProjectDialog, {
      props: {
        isOpen: true,
        ...mockHandlers,
      },
    });

    // Fill in the form
    const nameInput = screen.getByLabelText("Project Name");
    const pathInput = screen.getByLabelText("Project Directory");

    await fireEvent.input(nameInput, { target: { value: "Test Project" } });
    await fireEvent.input(pathInput, { target: { value: "/home/user/test-project" } });

    // Submit the form
    const createButton = screen.getByText("Create Project");
    await fireEvent.click(createButton);

    // Wait for async operations
    await waitFor(() => {
      expect(mockHandlers.onProjectCreated).toHaveBeenCalledWith(newProject);
      expect(mockHandlers.onClose).toHaveBeenCalled();
    });
  });

  it("should show error when project creation fails", async () => {
    mockIPC((cmd) => {
      if (cmd === "create_project") {
        throw { type: "AlreadyExists", message: "Directory already exists" };
      }
    });

    render(CreateProjectDialog, {
      props: {
        isOpen: true,
        ...mockHandlers,
      },
    });

    // Fill in the form
    const nameInput = screen.getByLabelText("Project Name");
    const pathInput = screen.getByLabelText("Project Directory");

    await fireEvent.input(nameInput, { target: { value: "Test Project" } });
    await fireEvent.input(pathInput, { target: { value: "/existing/path" } });

    // Submit the form
    const createButton = screen.getByText("Create Project");
    await fireEvent.click(createButton);

    // Wait for error message
    await waitFor(() => {
      expect(screen.getByText("Already exists: Directory already exists")).toBeInTheDocument();
    });

    expect(mockHandlers.onProjectCreated).not.toHaveBeenCalled();
    expect(mockHandlers.onClose).not.toHaveBeenCalled();
  });

  it("should handle directory selection", async () => {
    const { open } = await import("@tauri-apps/plugin-dialog");
    vi.mocked(open).mockResolvedValue("/selected/directory");

    render(CreateProjectDialog, {
      props: {
        isOpen: true,
        ...mockHandlers,
      },
    });

    const browseButton = screen.getByText("Browse");
    await fireEvent.click(browseButton);

    await waitFor(() => {
      const pathInput = screen.getByLabelText("Project Directory") as HTMLInputElement;
      expect(pathInput.value).toBe("/selected/directory");

      // Should auto-fill project name from directory
      const nameInput = screen.getByLabelText("Project Name") as HTMLInputElement;
      expect(nameInput.value).toBe("directory");
    });
  });

  it("should show loading state while creating project", async () => {
    let resolveCreate: (value: any) => void;
    const createPromise = new Promise((resolve) => {
      resolveCreate = resolve;
    });

    mockIPC((cmd) => {
      if (cmd === "create_project") {
        return createPromise;
      }
    });

    render(CreateProjectDialog, {
      props: {
        isOpen: true,
        ...mockHandlers,
      },
    });

    // Fill form
    await fireEvent.input(screen.getByLabelText("Project Name"), {
      target: { value: "Test" },
    });
    await fireEvent.input(screen.getByLabelText("Project Directory"), {
      target: { value: "/test" },
    });

    const createButton = screen.getByText("Create Project");
    await fireEvent.click(createButton);

    // Check loading state
    expect(screen.getByText("Creating...")).toBeInTheDocument();
    expect(createButton).toBeDisabled();

    // Resolve the promise
    resolveCreate!({
      id: 1,
      name: "Test",
      path: "/test",
      createdAt: "2024-01-20T00:00:00Z",
      updatedAt: "2024-01-20T00:00:00Z",
    });

    await waitFor(() => {
      expect(mockHandlers.onProjectCreated).toHaveBeenCalled();
    });
  });

  it("should call onClose when cancel is clicked", async () => {
    render(CreateProjectDialog, {
      props: {
        isOpen: true,
        ...mockHandlers,
      },
    });

    const cancelButton = screen.getByText("Cancel");
    await fireEvent.click(cancelButton);

    expect(mockHandlers.onClose).toHaveBeenCalled();
  });
});
