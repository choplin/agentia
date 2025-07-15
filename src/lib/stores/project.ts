import { writable, derived } from "svelte/store";
import { invoke } from "@tauri-apps/api/core";

export interface Project {
  id: string;
  path: string;
  name: string;
  created_at: string;
  updated_at: string;
  settings?: Record<string, any>;
}

// Projects store
function createProjectsStore() {
  const { subscribe, set, update } = writable<Project[]>([]);

  return {
    subscribe,
    async load() {
      try {
        const projects = await invoke<Project[]>("get_projects");
        set(projects);
        return projects;
      } catch (error) {
        console.error("Failed to load projects:", error);
        throw error;
      }
    },
    async create(name: string, path: string) {
      try {
        const project = await invoke<Project>("create_project", { name, path });
        update((projects) => [...projects, project]);
        return project;
      } catch (error) {
        console.error("Failed to create project:", error);
        throw error;
      }
    },
  };
}

// Current project store
function createCurrentProjectStore() {
  const { subscribe, set } = writable<Project | null>(null);

  return {
    subscribe,
    async select(projectId: string) {
      try {
        await invoke("select_project", { projectId });
        const projects = await invoke<Project[]>("get_projects");
        const selected = projects.find((p) => p.id === projectId);
        if (selected) {
          set(selected);
        }
        return selected;
      } catch (error) {
        console.error("Failed to select project:", error);
        throw error;
      }
    },
    set,
  };
}

export const projects = createProjectsStore();
export const currentProject = createCurrentProjectStore();

// Derived store for quick project lookup
export const projectsMap = derived(projects, ($projects) => {
  return new Map($projects.map((p) => [p.id, p]));
});
