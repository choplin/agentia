import { describe, it, expect, beforeEach } from "vitest";
import { get } from "svelte/store";
import { mockIPC, clearMocks } from "@tauri-apps/api/mocks";
import { projects, currentProject, projectsMap } from "./project";

describe("Project Store", () => {
  const mockProjectList = [
    {
      id: 1,
      name: "Project 1",
      path: "/path/to/project1",
      createdAt: "2024-01-01T00:00:00Z",
      updatedAt: "2024-01-01T00:00:00Z",
    },
    {
      id: 2,
      name: "Project 2",
      path: "/path/to/project2",
      createdAt: "2024-01-02T00:00:00Z",
      updatedAt: "2024-01-02T00:00:00Z",
    },
  ];

  beforeEach(() => {
    clearMocks();
    // Reset store state by reloading with empty data
    mockIPC((cmd) => {
      if (cmd === "get_projects") {
        return [];
      }
    });
    projects.load();
    currentProject.set(null);
  });

  describe("projects store", () => {
    it("should load projects successfully", async () => {
      mockIPC((cmd) => {
        if (cmd === "get_projects") {
          return mockProjectList;
        }
      });

      const result = await projects.load();
      expect(result).toEqual(mockProjectList);
      expect(get(projects)).toEqual(mockProjectList);
    });

    it("should handle load error", async () => {
      mockIPC((cmd) => {
        if (cmd === "get_projects") {
          throw new Error("Failed to load projects");
        }
      });

      await expect(projects.load()).rejects.toThrow("Failed to load projects");
    });

    it("should create a new project", async () => {
      const newProject = {
        id: 3,
        name: "New Project",
        path: "/path/to/new",
        createdAt: "2024-01-03T00:00:00Z",
        updatedAt: "2024-01-03T00:00:00Z",
      };

      mockIPC((cmd, args) => {
        if (cmd === "get_projects") {
          return mockProjectList;
        }
        if (cmd === "create_project") {
          expect(args).toEqual({ name: "New Project", path: "/path/to/new" });
          return newProject;
        }
      });

      // Load initial projects
      await projects.load();

      // Create new project
      const result = await projects.create("New Project", "/path/to/new");
      expect(result).toEqual(newProject);

      // Check that the new project was added to the store
      const allProjects = get(projects);
      expect(allProjects).toHaveLength(3);
      expect(allProjects).toContainEqual(newProject);
    });

    it("should handle create error", async () => {
      mockIPC((cmd) => {
        if (cmd === "create_project") {
          throw { type: "AlreadyExists", message: "Project already exists" };
        }
      });

      await expect(projects.create("Duplicate", "/existing/path")).rejects.toEqual({
        type: "AlreadyExists",
        message: "Project already exists",
      });
    });
  });

  describe("currentProject store", () => {
    it("should select a project", async () => {
      mockIPC((cmd, args) => {
        if (cmd === "get_projects") {
          return mockProjectList;
        }
        if (cmd === "select_project") {
          expect(args).toEqual({ projectId: 2 });
          return undefined;
        }
      });

      const result = await currentProject.select(2);
      expect(result).toEqual(mockProjectList[1]);
      expect(get(currentProject)).toEqual(mockProjectList[1]);
    });

    it("should handle select error", async () => {
      mockIPC((cmd) => {
        if (cmd === "select_project") {
          throw new Error("Permission denied");
        }
      });

      await expect(currentProject.select(1)).rejects.toThrow("Permission denied");
    });

    it("should return undefined when project not found", async () => {
      mockIPC((cmd) => {
        if (cmd === "get_projects") {
          return mockProjectList;
        }
        if (cmd === "select_project") {
          return undefined;
        }
      });

      const result = await currentProject.select(999);
      expect(result).toBeUndefined();
      expect(get(currentProject)).toBeNull();
    });
  });

  describe("projectsMap derived store", () => {
    it("should create a map of projects by id", async () => {
      mockIPC((cmd) => {
        if (cmd === "get_projects") {
          return mockProjectList;
        }
      });

      await projects.load();
      const map = get(projectsMap);

      expect(map).toBeInstanceOf(Map);
      expect(map.size).toBe(2);
      expect(map.get(1)).toEqual(mockProjectList[0]);
      expect(map.get(2)).toEqual(mockProjectList[1]);
    });

    it("should update when projects change", async () => {
      mockIPC((cmd, _args) => {
        if (cmd === "get_projects") {
          return [];
        }
        if (cmd === "create_project") {
          return mockProjectList[0];
        }
      });

      await projects.load();
      let map = get(projectsMap);
      expect(map.size).toBe(0);

      // Add a project via create method
      await projects.create(mockProjectList[0].name, mockProjectList[0].path);
      map = get(projectsMap);
      expect(map.size).toBe(1);
      expect(map.get(1)).toEqual(mockProjectList[0]);
    });
  });
});
