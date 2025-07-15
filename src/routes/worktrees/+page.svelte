<script lang="ts">
  import { Button } from "$lib/components/ui/button";
  import * as Card from "$lib/components/ui/card";
  import { GitBranch, Plus, Trash2, Check } from "lucide-svelte";

  // Mock data - will be fetched from API later
  const worktrees = [
    {
      id: "1",
      branch: "main",
      path: "/Users/aki/workspace/agentia",
      isDefault: true,
      isActive: true,
      lastCommit: "e3e8e34 refactor: replace emoji icons",
      status: "clean",
    },
    {
      id: "2",
      branch: "feature/ui-redesign",
      path: "/Users/aki/workspace/agentia-ui-redesign",
      isDefault: false,
      isActive: false,
      lastCommit: "d799576 feat: implement new UI layout",
      status: "modified",
    },
    {
      id: "3",
      branch: "feature/payment",
      path: "/Users/aki/workspace/agentia-payment",
      isDefault: false,
      isActive: false,
      lastCommit: "ba6221b fix: payment validation",
      status: "clean",
    },
  ];

  function switchWorktree(id: string) {
    console.log("Switch to worktree:", id);
  }

  function deleteWorktree(id: string) {
    console.log("Delete worktree:", id);
  }

  function createWorktree() {
    console.log("Create new worktree");
  }
</script>

<div class="space-y-6">
  <div class="flex items-center justify-between">
    <h1 class="text-3xl font-bold">Worktrees</h1>
    <Button onclick={createWorktree}>
      <Plus class="mr-2 h-4 w-4" />
      New Worktree
    </Button>
  </div>

  <!-- Worktrees Table -->
  <Card.Root>
    <Card.Content class="p-0">
      <table class="w-full">
        <thead class="border-b">
          <tr>
            <th class="p-4 text-left text-sm font-medium">Branch</th>
            <th class="p-4 text-left text-sm font-medium">Path</th>
            <th class="p-4 text-left text-sm font-medium">Status</th>
            <th class="p-4 text-left text-sm font-medium">Latest Commit</th>
            <th class="p-4 text-left text-sm font-medium">State</th>
            <th class="p-4 text-right text-sm font-medium">Actions</th>
          </tr>
        </thead>
        <tbody>
          {#each worktrees as worktree}
            <tr class="border-b hover:bg-muted/50">
              <td class="p-4">
                <div class="flex items-center gap-2 font-medium">
                  <GitBranch class="h-4 w-4" />
                  {worktree.branch}
                  {#if worktree.isDefault}
                    <span class="text-xs text-muted-foreground">(default)</span>
                  {/if}
                </div>
              </td>
              <td class="p-4 text-sm">
                <code class="rounded bg-muted px-2 py-1 text-xs">{worktree.path}</code>
              </td>
              <td class="p-4">
                {#if worktree.isActive}
                  <span
                    class="inline-flex items-center gap-1 rounded-full bg-primary px-2 py-1 text-xs font-medium text-primary-foreground"
                  >
                    <Check class="h-3 w-3" />
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
              <td class="p-4 text-sm">
                <code class="rounded bg-muted px-2 py-1 text-xs">{worktree.lastCommit}</code>
              </td>
              <td class="p-4">
                <span
                  class="inline-flex items-center rounded-full px-2 py-1 text-xs font-medium {worktree.status ===
                  'clean'
                    ? 'bg-green-100 text-green-700'
                    : 'bg-yellow-100 text-yellow-700'}"
                >
                  {worktree.status === "clean" ? "Clean" : "Modified"}
                </span>
              </td>
              <td class="p-4">
                <div class="flex justify-end gap-2">
                  {#if !worktree.isActive}
                    <Button variant="outline" size="sm" onclick={() => switchWorktree(worktree.id)}>
                      Switch
                    </Button>
                  {/if}
                  {#if !worktree.isDefault}
                    <Button variant="ghost" size="sm" onclick={() => deleteWorktree(worktree.id)}>
                      <Trash2 class="mr-2 h-4 w-4" />
                      Delete
                    </Button>
                  {/if}
                </div>
              </td>
            </tr>
          {/each}
        </tbody>
      </table>
    </Card.Content>
  </Card.Root>
</div>
