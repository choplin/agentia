<script lang="ts">
  import * as Card from "$lib/components/ui/card";
  import { TrendingUp, Clock, MessageSquare, GitBranch } from "lucide-svelte";

  // Mock data - will be fetched from API later
  const stats = {
    totalSessions: 342,
    totalMessages: 5678,
    totalDuration: "145 hours",
    avgSessionDuration: "25 minutes",
    mostActiveWorktree: "main",
    mostActiveDay: "Monday",
  };

  const weeklyActivity = [
    { day: "Mon", sessions: 12, messages: 180 },
    { day: "Tue", sessions: 15, messages: 220 },
    { day: "Wed", sessions: 8, messages: 120 },
    { day: "Thu", sessions: 10, messages: 150 },
    { day: "Fri", sessions: 18, messages: 280 },
    { day: "Sat", sessions: 5, messages: 80 },
    { day: "Sun", sessions: 3, messages: 50 },
  ];

  const topWorktrees = [
    { name: "main", sessions: 120, percentage: 35 },
    { name: "feature/ui-redesign", sessions: 80, percentage: 23 },
    { name: "feature/payment", sessions: 60, percentage: 18 },
    { name: "fix/bugs", sessions: 50, percentage: 15 },
    { name: "Others", sessions: 32, percentage: 9 },
  ];
</script>

<div class="space-y-6">
  <h1 class="text-3xl font-bold">Statistics</h1>

  <!-- Summary Cards -->
  <div class="grid gap-4 md:grid-cols-2 lg:grid-cols-4">
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
        <Card.Title class="text-sm font-medium">Total Messages</Card.Title>
        <MessageSquare class="h-4 w-4 text-muted-foreground" />
      </Card.Header>
      <Card.Content>
        <div class="text-2xl font-bold">{stats.totalMessages}</div>
        <p class="text-xs text-muted-foreground">Sent & received</p>
      </Card.Content>
    </Card.Root>

    <Card.Root>
      <Card.Header class="flex flex-row items-center justify-between space-y-0 pb-2">
        <Card.Title class="text-sm font-medium">Total Usage Time</Card.Title>
        <Clock class="h-4 w-4 text-muted-foreground" />
      </Card.Header>
      <Card.Content>
        <div class="text-2xl font-bold">{stats.totalDuration}</div>
        <p class="text-xs text-muted-foreground">Cumulative</p>
      </Card.Content>
    </Card.Root>

    <Card.Root>
      <Card.Header class="flex flex-row items-center justify-between space-y-0 pb-2">
        <Card.Title class="text-sm font-medium">Average Session Time</Card.Title>
        <TrendingUp class="h-4 w-4 text-muted-foreground" />
      </Card.Header>
      <Card.Content>
        <div class="text-2xl font-bold">{stats.avgSessionDuration}</div>
        <p class="text-xs text-muted-foreground">Per session</p>
      </Card.Content>
    </Card.Root>
  </div>

  <!-- Weekly Activity -->
  <Card.Root>
    <Card.Header>
      <Card.Title>Weekly Activity</Card.Title>
      <Card.Description>Sessions and messages by day of week</Card.Description>
    </Card.Header>
    <Card.Content>
      <div class="space-y-4">
        {#each weeklyActivity as day}
          <div class="flex items-center gap-4">
            <div class="w-8 text-sm font-medium">{day.day}</div>
            <div class="flex-1">
              <div class="flex items-center gap-2 text-sm">
                <div class="h-6 bg-primary" style="width: {(day.sessions / 20) * 100}%"></div>
                <span>{day.sessions} sessions</span>
              </div>
              <div class="flex items-center gap-2 text-sm">
                <div class="h-6 bg-primary/50" style="width: {(day.messages / 300) * 100}%"></div>
                <span>{day.messages} messages</span>
              </div>
            </div>
          </div>
        {/each}
      </div>
    </Card.Content>
  </Card.Root>

  <!-- Top Worktrees -->
  <Card.Root>
    <Card.Header>
      <Card.Title>Worktree Usage</Card.Title>
      <Card.Description>Most used worktrees</Card.Description>
    </Card.Header>
    <Card.Content>
      <div class="space-y-4">
        {#each topWorktrees as worktree}
          <div class="flex items-center gap-4">
            <GitBranch class="h-4 w-4 text-muted-foreground" />
            <div class="flex-1">
              <div class="flex items-center justify-between mb-1">
                <span class="text-sm font-medium">{worktree.name}</span>
                <span class="text-sm text-muted-foreground">{worktree.sessions} sessions</span>
              </div>
              <div class="h-2 bg-muted rounded-full overflow-hidden">
                <div class="h-full bg-primary" style="width: {worktree.percentage}%"></div>
              </div>
            </div>
            <span class="text-sm font-medium">{worktree.percentage}%</span>
          </div>
        {/each}
      </div>
    </Card.Content>
  </Card.Root>
</div>
