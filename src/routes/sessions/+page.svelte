<script lang="ts">
  import { onMount } from "svelte";
  import { Button } from "$lib/components/ui/button";
  import * as Card from "$lib/components/ui/card";
  import { Trash2, Download, Search, Filter, PlayCircle } from "lucide-svelte";
  import { sessions } from "$lib/stores/session";
  import { goto } from "$app/navigation";

  let loading = $state(true);
  let error = $state<string | null>(null);
  let selectedSessions = $state(new Set<string>());
  let filterStatus = $state("all");
  let searchQuery = $state("");

  onMount(async () => {
    try {
      await sessions.load();
    } catch (err) {
      error = err instanceof Error ? err.message : "Failed to load sessions";
    } finally {
      loading = false;
    }
  });

  // Computed filtered sessions
  let filteredSessions = $derived(() => {
    let filtered = $sessions;

    // Filter by status
    if (filterStatus !== "all") {
      filtered = filtered.filter((s) => {
        if (filterStatus === "running") return s.status === "active";
        if (filterStatus === "stopped") return s.status === "completed" || s.status === "failed";
        return true;
      });
    }

    // Filter by search query
    if (searchQuery) {
      const query = searchQuery.toLowerCase();
      filtered = filtered.filter((s) => s.title.toLowerCase().includes(query));
    }

    return filtered;
  });

  // Helper function to format dates
  function formatDate(dateString: string) {
    const date = new Date(dateString);
    return date.toLocaleString();
  }

  // Helper function to calculate duration
  function calculateDuration(start: string, end?: string) {
    const startDate = new Date(start);
    const endDate = end ? new Date(end) : new Date();
    const diff = endDate.getTime() - startDate.getTime();

    const hours = Math.floor(diff / (1000 * 60 * 60));
    const minutes = Math.floor((diff % (1000 * 60 * 60)) / (1000 * 60));

    if (hours > 0) {
      return `${hours}h ${minutes}m`;
    }
    return `${minutes}m`;
  }

  function viewSession(sessionId: string) {
    goto(`/session/${sessionId}`);
  }

  function toggleSelection(id: string) {
    if (selectedSessions.has(id)) {
      selectedSessions.delete(id);
    } else {
      selectedSessions.add(id);
    }
    selectedSessions = new Set(selectedSessions);
  }

  function toggleSelectAll() {
    const filtered = filteredSessions();
    if (selectedSessions.size === filtered.length) {
      selectedSessions = new Set();
    } else {
      selectedSessions = new Set(filtered.map((s) => s.id));
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
        <p class="text-center text-muted-foreground">Loading sessions...</p>
      </Card.Content>
    </Card.Root>
  {:else if filteredSessions().length === 0}
    <Card.Root>
      <Card.Content class="pt-6">
        <p class="text-center text-muted-foreground">
          {searchQuery || filterStatus !== "all"
            ? "No sessions match your filters."
            : "No sessions yet. Start a new session to begin!"}
        </p>
      </Card.Content>
    </Card.Root>
  {:else}
    <!-- Sessions Table -->
    <Card.Root>
      <Card.Content class="p-0">
        <table class="w-full">
          <thead class="border-b">
            <tr>
              <th class="p-4 text-left">
                <input
                  type="checkbox"
                  checked={selectedSessions.size === filteredSessions().length &&
                    filteredSessions().length > 0}
                  onchange={toggleSelectAll}
                />
              </th>
              <th class="p-4 text-left text-sm font-medium">Session Title</th>
              <th class="p-4 text-left text-sm font-medium">Status</th>
              <th class="p-4 text-left text-sm font-medium">Model</th>
              <th class="p-4 text-left text-sm font-medium">Created</th>
              <th class="p-4 text-left text-sm font-medium">Duration</th>
              <th class="p-4 text-left text-sm font-medium">Messages</th>
              <th class="p-4 text-right text-sm font-medium">Actions</th>
            </tr>
          </thead>
          <tbody>
            {#each filteredSessions() as session}
              <tr
                class="border-b hover:bg-muted/50 cursor-pointer"
                onclick={() => viewSession(session.id)}
              >
                <td class="p-4" onclick={(e) => e.stopPropagation()}>
                  <input
                    type="checkbox"
                    checked={selectedSessions.has(session.id)}
                    onchange={() => toggleSelection(session.id)}
                  />
                </td>
                <td class="p-4 font-medium">{session.title}</td>
                <td class="p-4">
                  <span
                    class="inline-flex items-center rounded-full px-2 py-1 text-xs font-medium"
                    class:bg-green-100={session.status === "active"}
                    class:text-green-700={session.status === "active"}
                    class:bg-gray-100={session.status === "completed"}
                    class:text-gray-700={session.status === "completed"}
                    class:bg-red-100={session.status === "failed"}
                    class:text-red-700={session.status === "failed"}
                    class:bg-yellow-100={session.status === "paused"}
                    class:text-yellow-700={session.status === "paused"}
                  >
                    {session.status.charAt(0).toUpperCase() + session.status.slice(1)}
                  </span>
                </td>
                <td class="p-4 text-sm">{session.config.model.split("-").slice(0, 3).join("-")}</td>
                <td class="p-4 text-sm">{formatDate(session.createdAt)}</td>
                <td class="p-4 text-sm"
                  >{calculateDuration(session.createdAt, session.updatedAt)}</td
                >
                <td class="p-4 text-sm">{session.messages.length}</td>
                <td class="p-4" onclick={(e) => e.stopPropagation()}>
                  <div class="flex justify-end gap-2">
                    {#if session.status === "active"}
                      <Button variant="outline" size="sm" onclick={() => viewSession(session.id)}>
                        <PlayCircle class="h-4 w-4" />
                      </Button>
                    {:else}
                      <Button variant="ghost" size="sm" onclick={() => viewSession(session.id)}>
                        View
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
  {/if}
</div>
