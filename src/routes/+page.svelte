<script lang="ts">
  import { Button } from "$lib/components/ui/button";
  import * as Card from "$lib/components/ui/card";
  import { MessageSquare, GitBranch, Clock, Activity } from "lucide-svelte";
  import { goto } from "$app/navigation";

  // Mock data - will be fetched from API later
  const stats = {
    activeSessions: 3,
    totalSessions: 127,
    activeWorktrees: 5,
    recentActivity: "2 hours ago",
  };

  const quickActions = [
    { label: "Start New Session", action: () => console.log("Start new session") },
    { label: "Resume Last Session", action: () => console.log("Resume last session") },
    { label: "Change Project", action: () => goto("/projects") },
    { label: "Create Worktree", action: () => goto("/worktrees") },
  ];

  const recentActivities = [
    {
      type: "session",
      title: "Payment API Implementation",
      time: "2 hours ago",
      worktree: "feature/payment",
    },
    { type: "worktree", title: "Created feature/ui-redesign", time: "5 hours ago" },
    { type: "session", title: "Bug Fix #123", time: "Yesterday", worktree: "fix/cart-bug" },
    { type: "session", title: "Refactoring Discussion", time: "3 days ago", worktree: "main" },
  ];
</script>

<div class="space-y-6">
  <h1 class="text-3xl font-bold">Home</h1>

  <!-- Stats Cards -->
  <div class="grid gap-4 md:grid-cols-2 lg:grid-cols-4">
    <Card.Root>
      <Card.Header class="flex flex-row items-center justify-between space-y-0 pb-2">
        <Card.Title class="text-sm font-medium">Active Sessions</Card.Title>
        <MessageSquare class="h-4 w-4 text-muted-foreground" />
      </Card.Header>
      <Card.Content>
        <div class="text-2xl font-bold">{stats.activeSessions}</div>
        <p class="text-xs text-muted-foreground">Currently active</p>
      </Card.Content>
    </Card.Root>

    <Card.Root>
      <Card.Header class="flex flex-row items-center justify-between space-y-0 pb-2">
        <Card.Title class="text-sm font-medium">Total Sessions</Card.Title>
        <MessageSquare class="h-4 w-4 text-muted-foreground" />
      </Card.Header>
      <Card.Content>
        <div class="text-2xl font-bold">{stats.totalSessions}</div>
        <p class="text-xs text-muted-foreground">All time</p>
      </Card.Content>
    </Card.Root>

    <Card.Root>
      <Card.Header class="flex flex-row items-center justify-between space-y-0 pb-2">
        <Card.Title class="text-sm font-medium">Active Worktrees</Card.Title>
        <GitBranch class="h-4 w-4 text-muted-foreground" />
      </Card.Header>
      <Card.Content>
        <div class="text-2xl font-bold">{stats.activeWorktrees}</div>
        <p class="text-xs text-muted-foreground">Branches</p>
      </Card.Content>
    </Card.Root>

    <Card.Root>
      <Card.Header class="flex flex-row items-center justify-between space-y-0 pb-2">
        <Card.Title class="text-sm font-medium">Last Activity</Card.Title>
        <Clock class="h-4 w-4 text-muted-foreground" />
      </Card.Header>
      <Card.Content>
        <div class="text-2xl font-bold">{stats.recentActivity}</div>
        <p class="text-xs text-muted-foreground">Most recent activity</p>
      </Card.Content>
    </Card.Root>
  </div>

  <!-- Quick Actions -->
  <Card.Root>
    <Card.Header>
      <Card.Title>Quick Actions</Card.Title>
      <Card.Description>Quick access to common operations</Card.Description>
    </Card.Header>
    <Card.Content>
      <div class="grid gap-2 md:grid-cols-2">
        {#each quickActions as action}
          <Button variant="outline" onclick={action.action}>
            {action.label}
          </Button>
        {/each}
      </div>
    </Card.Content>
  </Card.Root>

  <!-- Recent Activity Timeline -->
  <Card.Root>
    <Card.Header>
      <Card.Title>Recent Activity</Card.Title>
      <Card.Description>Recent activity in your project</Card.Description>
    </Card.Header>
    <Card.Content>
      <div class="space-y-4">
        {#each recentActivities as activity}
          <div class="flex items-start gap-4">
            <div class="mt-0.5">
              {#if activity.type === "session"}
                <MessageSquare class="h-4 w-4 text-muted-foreground" />
              {:else}
                <GitBranch class="h-4 w-4 text-muted-foreground" />
              {/if}
            </div>
            <div class="flex-1">
              <p class="text-sm font-medium">{activity.title}</p>
              <p class="text-xs text-muted-foreground">
                {#if activity.worktree}
                  {activity.worktree} •
                {/if}
                {activity.time}
              </p>
            </div>
          </div>
        {/each}
      </div>
    </Card.Content>
  </Card.Root>
</div>
