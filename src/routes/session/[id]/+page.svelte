<script lang="ts">
  import { onMount, onDestroy } from "svelte";
  import { page } from "$app/stores";
  import { Button } from "$lib/components/ui/button";
  import { Input } from "$lib/components/ui/input";
  import * as Card from "$lib/components/ui/card";
  import { ArrowLeft, Download, Play, Pause, Send } from "lucide-svelte";
  import { goto } from "$app/navigation";
  import MarkdownRenderer from "$lib/components/MarkdownRenderer.svelte";
  import ToolMessage from "$lib/components/ToolMessage.svelte";
  import type { Session, Message } from "$lib/types";
  import { invoke } from "@tauri-apps/api/core";
  import { listen, type UnlistenFn } from "@tauri-apps/api/event";

  // Get session ID from route params
  let sessionId = $derived($page.params.id);

  let loading = $state(true);
  let error = $state<string | null>(null);
  let session = $state<Session | null>(null);
  let messageInput = $state("");
  let sending = $state(false);
  let messagesContainer = $state<HTMLDivElement>();
  let messageListener: UnlistenFn | null = null;

  async function loadSession(id: string) {
    loading = true;
    error = null;

    try {
      // Unsubscribe from previous session
      if (messageListener) {
        messageListener();
        messageListener = null;
      }

      // Load session data
      session = await invoke<Session>("get_session", { sessionId: id });

      // Subscribe to real-time message updates
      messageListener = await listen<Message>(`session-${id}-message`, (event) => {
        if (session) {
          session.messages = [...session.messages, event.payload];
          // Update session status based on message
          if (session.status.type !== "exited" && session.status.type !== "failed") {
            if (event.payload.role === "assistant") {
              session.status = { type: "running" };
            }
          }
        }
      });
    } catch (err) {
      error = err instanceof Error ? err.message : "Failed to load session";
      session = null;
    } finally {
      loading = false;
    }
  }

  // Load session when component mounts or when sessionId changes
  $effect(() => {
    if (sessionId) {
      loadSession(sessionId);
    }
  });

  onDestroy(() => {
    if (messageListener) {
      messageListener();
    }
  });

  // Auto-scroll to bottom when new messages arrive
  $effect(() => {
    if (session && messagesContainer) {
      messagesContainer.scrollTop = messagesContainer.scrollHeight;
    }
  });

  async function sendMessage() {
    if (!messageInput.trim() || sending) return;

    const message = messageInput;
    messageInput = "";
    sending = true;

    try {
      // If session is paused, it will be automatically resumed when sending a message
      await invoke("send_message", { sessionId, message });

      // Update local session status if it was exited
      if (session && session.status.type === "exited") {
        // Status will be updated via event
      }
    } catch (err) {
      error = err instanceof Error ? err.message : "Failed to send message";
      // Restore the message on error
      messageInput = message;
    } finally {
      sending = false;
    }
  }

  async function stopSession() {
    try {
      await invoke("stop_session", { sessionId });
      // Reload session to get updated status
      session = await invoke<Session>("get_session", { sessionId });
    } catch (err) {
      error = err instanceof Error ? err.message : "Failed to stop session";
    }
  }

  function exportSession() {
    // TODO: Implement export functionality
    console.log("Export session");
  }

  // Helper to get display content for message
  function getMessageText(message: Message): string {
    if (message.type === "text") {
      return message.content.text || "";
    } else if (message.type === "error") {
      return `Error: ${message.content.error || "Unknown error"}`;
    }
    return "";
  }

  // Helper to check if message should show tool content
  function shouldShowToolContent(message: Message): boolean {
    return message.type === "toolUse" || message.type === "toolResult";
  }

  // Helper to format timestamp
  function formatTimestamp(timestamp: string): string {
    const date = new Date(timestamp);
    return date.toLocaleTimeString([], { hour: "2-digit", minute: "2-digit" });
  }
</script>

<div class="space-y-6">
  {#if loading}
    <Card.Root>
      <Card.Content class="pt-6">
        <p class="text-center text-muted-foreground">Loading session...</p>
      </Card.Content>
    </Card.Root>
  {:else if error}
    <Card.Root class="bg-destructive/10">
      <Card.Content class="pt-6">
        <p class="text-sm text-destructive">{error}</p>
      </Card.Content>
    </Card.Root>
  {:else if session}
    <!-- Header -->
    <div class="flex items-center justify-between">
      <div class="flex items-center gap-4">
        <Button variant="ghost" size="icon" onclick={() => goto("/sessions")}>
          <ArrowLeft class="h-4 w-4" />
        </Button>
        <div>
          <h1 class="text-2xl font-bold">{session.title}</h1>
          <p class="text-sm text-muted-foreground">
            ID: <span class="font-mono">{session.id.slice(0, 8)}</span> •
            {session.config.model} • Created {new Date(session.createdAt).toLocaleString()}
          </p>
        </div>
      </div>
      <div class="flex gap-2">
        <Button variant="outline" size="sm" onclick={exportSession}>
          <Download class="mr-2 h-4 w-4" />
          Export
        </Button>
        {#if session.status.type === "running"}
          <Button variant="destructive" size="sm" onclick={stopSession}>
            <Pause class="mr-2 h-4 w-4" />
            Stop
          </Button>
        {/if}
      </div>
    </div>

    <!-- Messages -->
    <Card.Root class="flex-1">
      <Card.Header>
        <Card.Title>Conversation History</Card.Title>
        <Card.Description>{session.messages.length} messages</Card.Description>
      </Card.Header>
      <Card.Content>
        <div bind:this={messagesContainer} class="space-y-4 max-h-[60vh] overflow-y-auto">
          {#each session.messages as message}
            <div class="rounded-lg p-4 {message.role === 'user' ? 'bg-muted' : 'bg-primary/10'}">
              <div class="mb-2 flex items-center justify-between">
                <span class="text-sm font-medium">
                  {message.role === "user" ? "User" : "Assistant"}
                </span>
                <span class="text-xs text-muted-foreground"
                  >{formatTimestamp(message.timestamp)}</span
                >
              </div>

              {#if shouldShowToolContent(message)}
                <!-- Show tool use/results -->
                <ToolMessage content={message.content} isToolUse={message.type === "toolUse"} />
              {:else}
                <!-- Simple text or error message -->
                <MarkdownRenderer content={getMessageText(message)} />
              {/if}
            </div>
          {/each}
        </div>
      </Card.Content>

      <Card.Footer>
        <form
          onsubmit={(e) => {
            e.preventDefault();
            sendMessage();
          }}
          class="flex w-full gap-2"
        >
          <Input
            type="text"
            bind:value={messageInput}
            placeholder="Type a message..."
            class="flex-1"
            disabled={sending}
          />
          <Button type="submit" disabled={sending || !messageInput.trim()}>
            <Send class="h-4 w-4" />
          </Button>
        </form>
      </Card.Footer>
    </Card.Root>
  {/if}
</div>
