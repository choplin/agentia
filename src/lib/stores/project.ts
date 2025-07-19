import { writable, derived } from "svelte/store";
import { getProjects, createProject, selectProject } from "$lib/api/tauri";
import type { Project } from "$lib/types";

// Projects store
function createProjectsStore() {
  const { subscribe, set, update } = writable<Project[]>([]);

  return {
    subscribe,
    async load() {
      try {
        const projectsList = await getProjects();
        set(projectsList);
        return projectsList;
      } catch (error) {
        console.error("Failed to load projects:", error);
        throw error;
      }
    },
    async create(name: string, path: string) {
      try {
        const project = await createProject(name, path);
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
    async select(projectId: number) {
      try {
        await selectProject(projectId);
        const projectsList = await getProjects();
        const selected = projectsList.find((p) => p.id === projectId);
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
