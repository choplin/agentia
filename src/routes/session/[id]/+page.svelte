<script lang="ts">
  import { page } from "$app/stores";
  import { Button } from "$lib/components/ui/button";
  import * as Card from "$lib/components/ui/card";
  import { ArrowLeft, Download, Play, Pause } from "lucide-svelte";
  import { goto } from "$app/navigation";

  // Get session ID from route params
  $: sessionId = $page.params.id;

  // Mock data - will be fetched from API later
  const session = {
    id: sessionId,
    name: "Payment API Implementation",
    status: "running",
    worktree: "feature/payment",
    startTime: "2024-01-15 10:30",
    duration: "2h 15m",
    messages: [
      {
        role: "user",
        content: "Please help me implement a payment API. We'll use Stripe.",
        timestamp: "10:30",
      },
      {
        role: "assistant",
        content:
          "I'll help you implement a payment API using Stripe. First, let's install the necessary packages.",
        timestamp: "10:31",
      },
      {
        role: "user",
        content: "Yes, please. I'd like to implement it in TypeScript.",
        timestamp: "10:32",
      },
    ],
  };

  function toggleSession() {
    console.log("Toggle session:", session.status);
  }

  function exportSession() {
    console.log("Export session");
  }
</script>

<div class="space-y-6">
  <!-- Header -->
  <div class="flex items-center justify-between">
    <div class="flex items-center gap-4">
      <Button variant="ghost" size="icon" onclick={() => goto("/sessions")}>
        <ArrowLeft class="h-4 w-4" />
      </Button>
      <div>
        <h1 class="text-2xl font-bold">{session.name}</h1>
        <p class="text-sm text-muted-foreground">
          {session.worktree} • {session.startTime} • {session.duration}
        </p>
      </div>
    </div>
    <div class="flex gap-2">
      <Button variant="outline" size="sm" onclick={exportSession}>
        <Download class="mr-2 h-4 w-4" />
        Export
      </Button>
      <Button
        variant={session.status === "running" ? "destructive" : "default"}
        size="sm"
        onclick={toggleSession}
      >
        {#if session.status === "running"}
          <Pause class="mr-2 h-4 w-4" />
          Stop
        {:else}
          <Play class="mr-2 h-4 w-4" />
          Resume
        {/if}
      </Button>
    </div>
  </div>

  <!-- Messages -->
  <Card.Root>
    <Card.Header>
      <Card.Title>Conversation History</Card.Title>
      <Card.Description>{session.messages.length} messages</Card.Description>
    </Card.Header>
    <Card.Content>
      <div class="space-y-4">
        {#each session.messages as message}
          <div class="rounded-lg p-4 {message.role === 'user' ? 'bg-muted' : 'bg-primary/10'}">
            <div class="mb-2 flex items-center justify-between">
              <span class="text-sm font-medium">
                {message.role === "user" ? "User" : "Assistant"}
              </span>
              <span class="text-xs text-muted-foreground">{message.timestamp}</span>
            </div>
            <p class="text-sm">{message.content}</p>
          </div>
        {/each}
      </div>
    </Card.Content>
  </Card.Root>
</div>
