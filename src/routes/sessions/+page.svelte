<script lang="ts">
  import { Button } from "$lib/components/ui/button";
  import * as Card from "$lib/components/ui/card";
  import { Trash2, Download, Search, Filter } from "lucide-svelte";

  // Mock data - will be fetched from API later
  const sessions = [
    {
      id: "1",
      name: "Payment API Implementation",
      status: "running",
      worktree: "feature/payment",
      startTime: "2024-01-15 10:30",
      duration: "2h 15m",
      messages: 45,
    },
    {
      id: "2",
      name: "Bug Fix #123",
      status: "stopped",
      worktree: "fix/cart-bug",
      startTime: "2024-01-14 14:20",
      duration: "1h 30m",
      messages: 23,
    },
    {
      id: "3",
      name: "Refactoring Discussion",
      status: "stopped",
      worktree: "main",
      startTime: "2024-01-12 09:00",
      duration: "3h 45m",
      messages: 67,
    },
  ];

  let selectedSessions = $state(new Set<string>());
  let filterStatus = $state("all");
  let searchQuery = $state("");

  function toggleSelection(id: string) {
    if (selectedSessions.has(id)) {
      selectedSessions.delete(id);
    } else {
      selectedSessions.add(id);
    }
    selectedSessions = new Set(selectedSessions);
  }

  function toggleSelectAll() {
    if (selectedSessions.size === sessions.length) {
      selectedSessions = new Set();
    } else {
      selectedSessions = new Set(sessions.map((s) => s.id));
    }
  }
</script>

<div class="space-y-6">
  <div class="flex items-center justify-between">
    <h1 class="text-3xl font-bold">Sessions</h1>
    <div class="flex gap-2">
      <Button variant="outline" size="sm">
        <Download class="mr-2 h-4 w-4" />
        Export
      </Button>
      <Button variant="outline" size="sm" disabled={selectedSessions.size === 0}>
        <Trash2 class="mr-2 h-4 w-4" />
        Delete ({selectedSessions.size})
      </Button>
    </div>
  </div>

  <!-- Filters and Search -->
  <Card.Root>
    <Card.Content class="p-4">
      <div class="flex gap-4">
        <div class="relative flex-1">
          <Search class="absolute left-3 top-1/2 h-4 w-4 -translate-y-1/2 text-muted-foreground" />
          <input
            type="text"
            placeholder="Search sessions..."
            class="h-10 w-full rounded-md border border-input bg-background pl-10 pr-3 text-sm"
            bind:value={searchQuery}
          />
        </div>
        <select
          class="h-10 rounded-md border border-input bg-background px-3 text-sm"
          bind:value={filterStatus}
        >
          <option value="all">All</option>
          <option value="running">Running</option>
          <option value="stopped">Stopped</option>
        </select>
        <Button variant="outline" size="icon">
          <Filter class="h-4 w-4" />
        </Button>
      </div>
    </Card.Content>
  </Card.Root>

  <!-- Sessions Table -->
  <Card.Root>
    <Card.Content class="p-0">
      <table class="w-full">
        <thead class="border-b">
          <tr>
            <th class="p-4 text-left">
              <input
                type="checkbox"
                checked={selectedSessions.size === sessions.length}
                onchange={toggleSelectAll}
              />
            </th>
            <th class="p-4 text-left text-sm font-medium">Session Name</th>
            <th class="p-4 text-left text-sm font-medium">Status</th>
            <th class="p-4 text-left text-sm font-medium">Worktree</th>
            <th class="p-4 text-left text-sm font-medium">Start Time</th>
            <th class="p-4 text-left text-sm font-medium">Duration</th>
            <th class="p-4 text-left text-sm font-medium">Messages</th>
          </tr>
        </thead>
        <tbody>
          {#each sessions as session}
            <tr class="border-b hover:bg-muted/50">
              <td class="p-4">
                <input
                  type="checkbox"
                  checked={selectedSessions.has(session.id)}
                  onchange={() => toggleSelection(session.id)}
                />
              </td>
              <td class="p-4 font-medium">{session.name}</td>
              <td class="p-4">
                <span
                  class="inline-flex items-center rounded-full px-2 py-1 text-xs font-medium"
                  class:bg-green-100={session.status === "running"}
                  class:text-green-700={session.status === "running"}
                  class:bg-gray-100={session.status === "stopped"}
                  class:text-gray-700={session.status === "stopped"}
                >
                  {session.status === "running" ? "Running" : "Stopped"}
                </span>
              </td>
              <td class="p-4 text-sm">{session.worktree}</td>
              <td class="p-4 text-sm">{session.startTime}</td>
              <td class="p-4 text-sm">{session.duration}</td>
              <td class="p-4 text-sm">{session.messages}</td>
            </tr>
          {/each}
        </tbody>
      </table>
    </Card.Content>
  </Card.Root>
</div>
