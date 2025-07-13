<script lang="ts">
  import { onMount } from "svelte";
  import type { Project } from "$lib/claude";

  interface Props {
    projects: Project[];
    currentProject: Project | null;
    onProjectSelect: (project: Project) => void;
    onProjectCreate: () => void;
  }

  let { projects, currentProject, onProjectSelect, onProjectCreate }: Props = $props();

  let isOpen = $state(false);
</script>

<div class="relative">
  <button
    onclick={() => (isOpen = !isOpen)}
    class="flex items-center gap-2 px-3 py-2 border border-border rounded-md hover:bg-accent/50 transition-colors"
  >
    <div class="text-sm">
      <div class="font-medium">{currentProject?.name || "Select Project"}</div>
      {#if currentProject}
        <div class="text-xs text-muted-foreground">{currentProject.path}</div>
      {/if}
    </div>
    <svg class="w-4 h-4 ml-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7" />
    </svg>
  </button>

  {#if isOpen}
    <div
      class="absolute top-full left-0 mt-1 w-80 bg-card border border-border rounded-md shadow-lg z-50"
    >
      <div class="p-2">
        <button
          onclick={onProjectCreate}
          class="w-full px-3 py-2 text-sm bg-primary text-primary-foreground rounded hover:bg-primary/90 transition-colors"
        >
          Create New Project
        </button>
      </div>

      <div class="border-t border-border">
        <div class="max-h-64 overflow-y-auto">
          {#each projects as project}
            <button
              onclick={() => {
                onProjectSelect(project);
                isOpen = false;
              }}
              class="w-full px-3 py-2 text-left hover:bg-accent hover:text-accent-foreground transition-colors {currentProject?.id ===
              project.id
                ? 'bg-accent/50'
                : ''}"
            >
              <div class="text-sm font-medium">{project.name}</div>
              <div class="text-xs text-muted-foreground">{project.path}</div>
            </button>
          {/each}
        </div>
      </div>
    </div>
  {/if}
</div>

{#if isOpen}
  <button
    onclick={() => (isOpen = false)}
    class="fixed inset-0 z-40"
    aria-label="Close project selector"
  ></button>
{/if}
