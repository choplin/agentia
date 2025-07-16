<script lang="ts">
  import { onMount } from "svelte";
  import { Button } from "$lib/components/ui/button";
  import * as Card from "$lib/components/ui/card";
  import * as Table from "$lib/components/ui/table";
  import { Input } from "$lib/components/ui/input";
  import { Badge } from "$lib/components/ui/badge";
  import { Checkbox } from "$lib/components/ui/checkbox";
  import { SimpleTooltip } from "$lib/components/ui/tooltip";
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
      <SimpleTooltip content="Export sessions to CSV">
        {#snippet children()}
          <Button variant="outline" size="sm">
            <Download class="mr-2 h-4 w-4" />
            Export
          </Button>
        {/snippet}
      </SimpleTooltip>
      <SimpleTooltip content="Delete selected sessions">
        {#snippet children()}
          <Button variant="outline" size="sm" disabled={selectedSessions.size === 0}>
            <Trash2 class="mr-2 h-4 w-4" />
            Delete ({selectedSessions.size})
          </Button>
        {/snippet}
      </SimpleTooltip>
    </div>
  </div>

  <!-- Filters and Search -->
  <Card.Root>
    <Card.Content class="p-4">
      <div class="flex gap-4">
        <div class="relative flex-1">
          <Search
            class="absolute left-3 top-1/2 h-4 w-4 -translate-y-1/2 text-muted-foreground pointer-events-none"
          />
          <Input
            type="text"
            placeholder="Search sessions..."
            class="pl-10"
            bind:value={searchQuery}
          />
        </div>
        <select
          class="h-10 rounded-md border border-input bg-background px-3 text-sm hover:border-primary/50 hover:shadow-sm focus:border-ring focus:outline-none focus:ring-2 focus:ring-ring/20 transition-all cursor-pointer"
          bind:value={filterStatus}
        >
          <option value="all">All</option>
          <option value="running">Running</option>
          <option value="stopped">Stopped</option>
        </select>
        <SimpleTooltip content="More filters">
          {#snippet children()}
            <Button variant="outline" size="icon">
              <Filter class="h-4 w-4" />
            </Button>
          {/snippet}
        </SimpleTooltip>
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
        <Table.Root>
          <Table.Header>
            <Table.Row>
              <Table.Head class="w-12">
                <Checkbox
                  checked={selectedSessions.size === filteredSessions().length &&
                    filteredSessions().length > 0}
                  onCheckedChange={toggleSelectAll}
                />
              </Table.Head>
              <Table.Head>Session Title</Table.Head>
              <Table.Head>Status</Table.Head>
              <Table.Head>Model</Table.Head>
              <Table.Head>Created</Table.Head>
              <Table.Head>Duration</Table.Head>
              <Table.Head>Messages</Table.Head>
              <Table.Head class="text-right">Actions</Table.Head>
            </Table.Row>
          </Table.Header>
          <Table.Body>
            {#each filteredSessions() as session}
              <Table.Row onclick={() => viewSession(session.id)}>
                <Table.Cell onclick={(e) => e.stopPropagation()}>
                  <Checkbox
                    checked={selectedSessions.has(session.id)}
                    onCheckedChange={() => toggleSelection(session.id)}
                  />
                </Table.Cell>
                <Table.Cell class="font-medium">{session.title}</Table.Cell>
                <Table.Cell>
                  <SimpleTooltip content={`Status: ${session.status}`}>
                    {#snippet children()}
                      <Badge
                        variant={session.status === "active"
                          ? "default"
                          : session.status === "failed"
                            ? "destructive"
                            : "secondary"}
                      >
                        {session.status.charAt(0).toUpperCase() + session.status.slice(1)}
                      </Badge>
                    {/snippet}
                  </SimpleTooltip>
                </Table.Cell>
                <Table.Cell>{session.config.model.split("-").slice(0, 3).join("-")}</Table.Cell>
                <Table.Cell>{formatDate(session.createdAt)}</Table.Cell>
                <Table.Cell>{calculateDuration(session.createdAt, session.updatedAt)}</Table.Cell>
                <Table.Cell>{session.messages.length}</Table.Cell>
                <Table.Cell class="text-right" onclick={(e) => e.stopPropagation()}>
                  {#if session.status === "active"}
                    <SimpleTooltip content="Continue session">
                      {#snippet children()}
                        <Button variant="outline" size="sm" onclick={() => viewSession(session.id)}>
                          <PlayCircle class="h-4 w-4" />
                        </Button>
                      {/snippet}
                    </SimpleTooltip>
                  {:else}
                    <SimpleTooltip content="View session details">
                      {#snippet children()}
                        <Button variant="ghost" size="sm" onclick={() => viewSession(session.id)}>
                          View
                        </Button>
                      {/snippet}
                    </SimpleTooltip>
                  {/if}
                </Table.Cell>
              </Table.Row>
            {/each}
          </Table.Body>
        </Table.Root>
      </Card.Content>
    </Card.Root>
  {/if}
</div>
