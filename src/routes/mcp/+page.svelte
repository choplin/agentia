<script lang="ts">
  import { Button } from "$lib/components/ui/button";
  import * as Card from "$lib/components/ui/card";
  import { Plug, Plus, Settings, Power } from "lucide-svelte";

  // Mock data - will be fetched from API later
  const mcpServers = [
    {
      id: "1",
      name: "File System MCP",
      description: "MCP server for file system operations",
      status: "connected",
      version: "1.0.0",
      capabilities: ["read", "write", "delete"],
    },
    {
      id: "2",
      name: "Git MCP",
      description: "MCP server for Git operations",
      status: "disconnected",
      version: "1.2.0",
      capabilities: ["commit", "push", "pull", "branch"],
    },
    {
      id: "3",
      name: "Database MCP",
      description: "MCP server for database operations",
      status: "error",
      version: "0.9.0",
      capabilities: ["query", "insert", "update", "delete"],
    },
  ];

  function toggleConnection(id: string) {
    console.log("Toggle connection:", id);
  }

  function configureMCP(id: string) {
    console.log("Configure MCP:", id);
  }

  function addMCP() {
    console.log("Add new MCP");
  }
</script>

<div class="space-y-6">
  <div class="flex items-center justify-between">
    <h1 class="text-3xl font-bold">MCP Servers</h1>
    <Button onclick={addMCP}>
      <Plus class="mr-2 h-4 w-4" />
      Add MCP Server
    </Button>
  </div>

  <!-- MCP Servers Table -->
  <Card.Root>
    <Card.Content class="p-0">
      <table class="w-full">
        <thead class="border-b">
          <tr>
            <th class="p-4 text-left text-sm font-medium">Server Name</th>
            <th class="p-4 text-left text-sm font-medium">Description</th>
            <th class="p-4 text-left text-sm font-medium">Status</th>
            <th class="p-4 text-left text-sm font-medium">Version</th>
            <th class="p-4 text-left text-sm font-medium">Capabilities</th>
            <th class="p-4 text-right text-sm font-medium">Actions</th>
          </tr>
        </thead>
        <tbody>
          {#each mcpServers as server}
            <tr class="border-b hover:bg-muted/50">
              <td class="p-4">
                <div class="flex items-center gap-2 font-medium">
                  <Plug class="h-4 w-4" />
                  {server.name}
                </div>
              </td>
              <td class="p-4 text-sm text-muted-foreground">{server.description}</td>
              <td class="p-4">
                <span
                  class="inline-flex items-center rounded-full px-2 py-1 text-xs font-medium"
                  class:bg-green-100={server.status === "connected"}
                  class:text-green-700={server.status === "connected"}
                  class:bg-gray-100={server.status === "disconnected"}
                  class:text-gray-700={server.status === "disconnected"}
                  class:bg-red-100={server.status === "error"}
                  class:text-red-700={server.status === "error"}
                >
                  {#if server.status === "connected"}
                    Connected
                  {:else if server.status === "disconnected"}
                    Disconnected
                  {:else}
                    Error
                  {/if}
                </span>
              </td>
              <td class="p-4 text-sm">{server.version}</td>
              <td class="p-4">
                <div class="flex flex-wrap gap-1">
                  {#each server.capabilities as capability}
                    <span class="rounded bg-muted px-2 py-1 text-xs">{capability}</span>
                  {/each}
                </div>
              </td>
              <td class="p-4">
                <div class="flex justify-end gap-2">
                  <Button
                    variant={server.status === "connected" ? "destructive" : "default"}
                    size="sm"
                    onclick={() => toggleConnection(server.id)}
                  >
                    <Power class="mr-2 h-4 w-4" />
                    {server.status === "connected" ? "Disconnect" : "Connect"}
                  </Button>
                  <Button variant="outline" size="sm" onclick={() => configureMCP(server.id)}>
                    <Settings class="mr-2 h-4 w-4" />
                    Configure
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
