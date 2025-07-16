<script lang="ts">
  import { onMount } from "svelte";
  import { Button } from "$lib/components/ui/button";
  import * as Card from "$lib/components/ui/card";
  import * as Table from "$lib/components/ui/table";
  import { Input } from "$lib/components/ui/input";
  import { Badge } from "$lib/components/ui/badge";
  import { SimpleTooltip } from "$lib/components/ui/tooltip";
  import { Plus, Trash2, FolderOpen, ExternalLink } from "lucide-svelte";
  import { projects, currentProject } from "$lib/stores/project";
  import { open } from "@tauri-apps/plugin-dialog";
  import { openPath } from "@tauri-apps/plugin-opener";

  let loading = $state(true);
  let error = $state<string | null>(null);
  let showNewProjectDialog = $state(false);
  let newProjectName = $state("");
  let newProjectPath = $state("");

  onMount(async () => {
    try {
      await projects.load();
    } catch (err) {
      error = err instanceof Error ? err.message : "Failed to load projects";
    } finally {
      loading = false;
    }
  });

  async function selectProject(id: string) {
    try {
      await currentProject.select(id);
    } catch (err) {
      error = err instanceof Error ? err.message : "Failed to select project";
    }
  }

  function deleteProject(id: string) {
    // TODO: Implement delete project functionality
    console.log("Delete project:", id);
  }

  async function openInFinder(path: string) {
    try {
      await openPath(path);
    } catch (err) {
      error = err instanceof Error ? err.message : "Failed to open folder";
    }
  }

  async function selectProjectDirectory() {
    const selected = await open({
      directory: true,
      multiple: false,
      title: "Select Project Directory",
    });

    if (selected) {
      newProjectPath = selected;
      // Auto-fill project name from directory name
      const pathParts = selected.split("/");
      newProjectName = pathParts[pathParts.length - 1] || "";
    }
  }

  async function createNewProject() {
    if (!newProjectName || !newProjectPath) {
      error = "Project name and path are required";
      return;
    }

    try {
      await projects.create(newProjectName, newProjectPath);
      showNewProjectDialog = false;
      newProjectName = "";
      newProjectPath = "";
    } catch (err) {
      error = err instanceof Error ? err.message : "Failed to create project";
    }
  }

  // Helper function to format dates
  function formatDate(dateString: string) {
    const date = new Date(dateString);
    return date.toLocaleString();
  }
</script>

<div class="space-y-6">
  <div class="flex items-center justify-between">
    <h1 class="text-3xl font-bold">Projects</h1>
    <Button onclick={() => (showNewProjectDialog = true)}>
      <Plus class="mr-2 h-4 w-4" />
      New Project
    </Button>
  </div>

  {#if error}
    <Card.Root class="bg-destructive/10">
      <Card.Content class="pt-6">
        <p class="text-sm text-destructive">{error}</p>
      </Card.Content>
    </Card.Root>
  {/if}

  {#if loading}
    <Card.Root>
      <Card.Content class="pt-6">
        <p class="text-center text-muted-foreground">Loading projects...</p>
      </Card.Content>
    </Card.Root>
  {:else if $projects.length === 0}
    <Card.Root>
      <Card.Content class="pt-6">
        <p class="text-center text-muted-foreground">No projects yet. Create your first project!</p>
      </Card.Content>
    </Card.Root>
  {:else}
    <!-- Projects Table -->
    <Card.Root>
      <Card.Content class="p-0">
        <Table.Root>
          <Table.Header>
            <Table.Row>
              <Table.Head>Project Name</Table.Head>
              <Table.Head>Description</Table.Head>
              <Table.Head>Path</Table.Head>
              <Table.Head>Status</Table.Head>
              <Table.Head>Last Accessed</Table.Head>
              <Table.Head class="text-right">Actions</Table.Head>
            </Table.Row>
          </Table.Header>
          <Table.Body>
            {#each $projects as project}
              <Table.Row class="cursor-pointer">
                <Table.Cell onclick={() => selectProject(project.id)}>
                  <div class="flex items-center gap-2 font-medium">
                    <FolderOpen class="h-4 w-4" />
                    {project.name}
                  </div>
                </Table.Cell>
                <Table.Cell class="text-muted-foreground" onclick={() => selectProject(project.id)}>
                  {project.settings ? JSON.parse(project.settings).description || "-" : "-"}
                </Table.Cell>
                <Table.Cell onclick={() => selectProject(project.id)}>
                  <code class="rounded bg-muted px-2 py-1 text-xs">{project.path}</code>
                </Table.Cell>
                <Table.Cell onclick={() => selectProject(project.id)}>
                  <Badge variant={$currentProject?.id === project.id ? "default" : "secondary"}>
                    {$currentProject?.id === project.id ? "Active" : "Inactive"}
                  </Badge>
                </Table.Cell>
                <Table.Cell onclick={() => selectProject(project.id)}>
                  {formatDate(project.updatedAt)}
                </Table.Cell>
                <Table.Cell onclick={(e) => e.stopPropagation()}>
                  <div class="flex justify-end gap-2">
                    {#if $currentProject?.id !== project.id}
                      <Button
                        variant="outline"
                        size="sm"
                        onclick={() => selectProject(project.id)}
                        class="gap-2"
                      >
                        Select
                      </Button>
                    {/if}
                    <SimpleTooltip content="Open in file manager">
                      {#snippet children()}
                        <Button
                          variant="ghost"
                          size="icon"
                          onclick={() => openInFinder(project.path)}
                        >
                          <ExternalLink class="h-4 w-4" />
                        </Button>
                      {/snippet}
                    </SimpleTooltip>
                    <SimpleTooltip content="Delete project">
                      {#snippet children()}
                        <Button
                          variant="ghost"
                          size="icon"
                          onclick={() => deleteProject(project.id)}
                        >
                          <Trash2 class="h-4 w-4" />
                        </Button>
                      {/snippet}
                    </SimpleTooltip>
                  </div>
                </Table.Cell>
              </Table.Row>
            {/each}
          </Table.Body>
        </Table.Root>
      </Card.Content>
    </Card.Root>
  {/if}

  <!-- New Project Dialog -->
  {#if showNewProjectDialog}
    <div class="fixed inset-0 z-50 flex items-center justify-center bg-black/50">
      <Card.Root class="w-[500px]">
        <Card.Header>
          <Card.Title>Create New Project</Card.Title>
        </Card.Header>
        <Card.Content class="space-y-4">
          <div>
            <label for="project-name" class="text-sm font-medium">Project Name</label>
            <Input
              id="project-name"
              type="text"
              bind:value={newProjectName}
              class="mt-1"
              placeholder="My Project"
            />
          </div>
          <div>
            <label for="project-path" class="text-sm font-medium">Project Path</label>
            <div class="mt-1 flex gap-2">
              <Input
                id="project-path"
                type="text"
                bind:value={newProjectPath}
                class="flex-1"
                placeholder="/path/to/project"
                readonly
              />
              <Button variant="outline" onclick={selectProjectDirectory}>Browse</Button>
            </div>
          </div>
        </Card.Content>
        <Card.Footer class="flex justify-end gap-2">
          <Button variant="outline" onclick={() => (showNewProjectDialog = false)}>Cancel</Button>
          <Button onclick={createNewProject}>Create Project</Button>
        </Card.Footer>
      </Card.Root>
    </div>
  {/if}
</div>
