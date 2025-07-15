<script lang="ts">
  import { Button } from "$lib/components/ui/button";
  import * as Card from "$lib/components/ui/card";
  import { Plus, Trash2, FolderOpen, ExternalLink } from "lucide-svelte";

  // Mock data - will be fetched from API later
  const projects = [
    {
      id: "1",
      name: "agentia",
      path: "/Users/aki/workspace/agentia",
      description: "AI Agent Management Tool",
      lastOpened: "2024-01-15 10:30",
      isActive: true,
    },
    {
      id: "2",
      name: "my-project",
      path: "/Users/aki/projects/my-project",
      description: "Personal Project",
      lastOpened: "2024-01-10 15:20",
      isActive: false,
    },
    {
      id: "3",
      name: "work-project",
      path: "/Users/aki/work/work-project",
      description: "Work Project",
      lastOpened: "2024-01-08 09:00",
      isActive: false,
    },
  ];

  function selectProject(id: string) {
    console.log("Select project:", id);
  }

  function deleteProject(id: string) {
    console.log("Delete project:", id);
  }

  function openInFinder(path: string) {
    console.log("Open in finder:", path);
  }
</script>

<div class="space-y-6">
  <div class="flex items-center justify-between">
    <h1 class="text-3xl font-bold">Projects</h1>
    <Button>
      <Plus class="mr-2 h-4 w-4" />
      New Project
    </Button>
  </div>

  <!-- Projects Table -->
  <Card.Root>
    <Card.Content class="p-0">
      <table class="w-full">
        <thead class="border-b">
          <tr>
            <th class="p-4 text-left text-sm font-medium">Project Name</th>
            <th class="p-4 text-left text-sm font-medium">Description</th>
            <th class="p-4 text-left text-sm font-medium">Path</th>
            <th class="p-4 text-left text-sm font-medium">Status</th>
            <th class="p-4 text-left text-sm font-medium">Last Accessed</th>
            <th class="p-4 text-right text-sm font-medium">Actions</th>
          </tr>
        </thead>
        <tbody>
          {#each projects as project}
            <tr class="border-b hover:bg-muted/50">
              <td class="p-4">
                <div class="flex items-center gap-2 font-medium">
                  <FolderOpen class="h-4 w-4" />
                  {project.name}
                </div>
              </td>
              <td class="p-4 text-sm text-muted-foreground">{project.description}</td>
              <td class="p-4 text-sm">
                <code class="rounded bg-muted px-2 py-1 text-xs">{project.path}</code>
              </td>
              <td class="p-4">
                {#if project.isActive}
                  <span
                    class="inline-flex items-center rounded-full bg-primary px-2 py-1 text-xs font-medium text-primary-foreground"
                  >
                    Active
                  </span>
                {:else}
                  <span
                    class="inline-flex items-center rounded-full bg-gray-100 px-2 py-1 text-xs font-medium text-gray-700"
                  >
                    Inactive
                  </span>
                {/if}
              </td>
              <td class="p-4 text-sm">{project.lastOpened}</td>
              <td class="p-4">
                <div class="flex justify-end gap-2">
                  {#if !project.isActive}
                    <Button variant="outline" size="sm" onclick={() => selectProject(project.id)}>
                      Select
                    </Button>
                  {/if}
                  <Button variant="ghost" size="icon" onclick={() => openInFinder(project.path)}>
                    <ExternalLink class="h-4 w-4" />
                  </Button>
                  <Button variant="ghost" size="icon" onclick={() => deleteProject(project.id)}>
                    <Trash2 class="h-4 w-4" />
                  </Button>
                </div>
              </td>
            </tr>
          {/each}
        </tbody>
      </table>
    </Card.Content>
  </Card.Root>
</div>
