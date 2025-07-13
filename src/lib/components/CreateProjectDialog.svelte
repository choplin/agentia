<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { open } from "@tauri-apps/plugin-dialog";

  interface Props {
    isOpen: boolean;
    onClose: () => void;
    onProjectCreated: (project: any) => void;
  }

  let { isOpen, onClose, onProjectCreated }: Props = $props();

  let projectName = $state("");
  let projectPath = $state("");
  let isCreating = $state(false);
  let error = $state("");

  async function selectDirectory() {
    try {
      const selected = await open({
        directory: true,
        multiple: false,
      });

      if (selected) {
        projectPath = selected as string;
        // Auto-fill project name from directory name if empty
        if (!projectName) {
          const parts = projectPath.split("/");
          projectName = parts[parts.length - 1] || parts[parts.length - 2] || "";
        }
      }
    } catch (err) {
      console.error("Failed to select directory:", err);
    }
  }

  async function createProject() {
    if (!projectName || !projectPath) {
      error = "Please provide both project name and path";
      return;
    }

    isCreating = true;
    error = "";

    try {
      const project = await invoke("create_project", {
        name: projectName,
        path: projectPath,
      });

      onProjectCreated(project);
      resetForm();
      onClose();
    } catch (err) {
      error = err as string;
    } finally {
      isCreating = false;
    }
  }

  function resetForm() {
    projectName = "";
    projectPath = "";
    error = "";
  }

  function handleClose() {
    resetForm();
    onClose();
  }
</script>

{#if isOpen}
  <div class="fixed inset-0 bg-black/50 flex items-center justify-center z-50">
    <div class="bg-card border border-border rounded-lg w-96 p-6">
      <h2 class="text-lg font-semibold mb-4">Create New Project</h2>

      <div class="space-y-4">
        <div>
          <label for="project-name" class="block text-sm font-medium mb-1"> Project Name </label>
          <input
            id="project-name"
            type="text"
            bind:value={projectName}
            placeholder="My Project"
            class="w-full px-3 py-2 border border-input bg-background rounded-md focus:outline-none focus:ring-2 focus:ring-ring"
          />
        </div>

        <div>
          <label for="project-path" class="block text-sm font-medium mb-1">
            Project Directory
          </label>
          <div class="flex gap-2">
            <input
              id="project-path"
              type="text"
              bind:value={projectPath}
              placeholder="/path/to/project"
              readonly
              class="flex-1 px-3 py-2 border border-input bg-background rounded-md focus:outline-none focus:ring-2 focus:ring-ring"
            />
            <button
              onclick={selectDirectory}
              class="px-4 py-2 border border-input rounded-md hover:bg-accent hover:text-accent-foreground transition-colors"
            >
              Browse
            </button>
          </div>
        </div>

        {#if error}
          <div class="text-sm text-destructive">{error}</div>
        {/if}
      </div>

      <div class="flex justify-end gap-2 mt-6">
        <button
          onclick={handleClose}
          disabled={isCreating}
          class="px-4 py-2 border border-input rounded-md hover:bg-accent hover:text-accent-foreground transition-colors disabled:opacity-50"
        >
          Cancel
        </button>
        <button
          onclick={createProject}
          disabled={isCreating || !projectName || !projectPath}
          class="px-4 py-2 bg-primary text-primary-foreground rounded-md hover:bg-primary/90 transition-colors disabled:opacity-50"
        >
          {isCreating ? "Creating..." : "Create Project"}
        </button>
      </div>
    </div>
  </div>
{/if}
