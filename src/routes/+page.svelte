<script lang="ts">
  import { onMount } from "svelte";
  import { ClaudeAPI } from "$lib/claude";
  import type { Session, Message } from "$lib/claude";

  const api = new ClaudeAPI();

  let sessions = $state<Session[]>([]);
  let selectedSession = $state<Session | null>(null);
  let messages = $state<Message[]>([]);
  let inputText = $state("");
  let isLoading = $state(false);

  onMount(async () => {
    await loadSessions();
  });

  async function loadSessions() {
    try {
      sessions = await api.listSessions();
      if (sessions.length > 0 && !selectedSession) {
        await selectSession(sessions[0]);
      }
    } catch (error) {
      console.error("Failed to load sessions:", error);
    }
  }

  async function createNewSession() {
    try {
      const session = await api.createSession(`Session ${sessions.length + 1}`);
      sessions = [session, ...sessions];
      await selectSession(session);
    } catch (error) {
      console.error("Failed to create session:", error);
    }
  }

  async function selectSession(session: Session) {
    selectedSession = session;
    const fullSession = await api.getSession(session.id);
    messages = fullSession.messages;

    // Set up listener for new messages
    await api.listenToSession(session.id, (message) => {
      messages = [...messages, message];
    });
  }

  async function sendMessage() {
    if (!selectedSession || !inputText.trim() || isLoading) return;

    isLoading = true;
    try {
      await api.sendMessage(selectedSession.id, inputText.trim());
      inputText = "";
    } catch (error) {
      console.error("Failed to send message:", error);
    } finally {
      isLoading = false;
    }
  }
</script>

<main class="flex h-screen bg-background text-foreground">
  <!-- Sidebar -->
  <aside class="w-64 border-r border-border bg-card flex flex-col">
    <div class="p-4 border-b border-border">
      <button
        onclick={createNewSession}
        class="w-full px-4 py-2 bg-primary text-primary-foreground rounded-md hover:bg-primary/90"
      >
        New Session
      </button>
    </div>

    <div class="flex-1 overflow-y-auto">
      {#each sessions as session}
        <button
          onclick={() => selectSession(session)}
          class="w-full px-4 py-3 text-left hover:bg-accent hover:text-accent-foreground transition-colors {selectedSession?.id ===
          session.id
            ? 'bg-accent text-accent-foreground'
            : ''}"
        >
          <div class="font-medium">{session.title}</div>
          <div class="text-sm text-muted-foreground">
            {new Date(session.createdAt).toLocaleDateString()}
          </div>
        </button>
      {/each}
    </div>
  </aside>

  <!-- Main Content -->
  <div class="flex-1 flex flex-col">
    <!-- Messages Area -->
    <div class="flex-1 overflow-y-auto p-4 space-y-4">
      {#if selectedSession}
        {#each messages as message}
          <div class="flex {message.role === 'user' ? 'justify-end' : 'justify-start'}">
            <div
              class="max-w-[70%] rounded-lg px-4 py-2 {message.role === 'user'
                ? 'bg-primary text-primary-foreground'
                : 'bg-muted'}"
            >
              {#if message.messageType === "text" && message.content.text}
                <p class="whitespace-pre-wrap">{message.content.text}</p>
              {:else if message.messageType === "error" && message.content.error}
                <p class="text-destructive">{message.content.error}</p>
              {:else if message.messageType === "toolUse" && message.content.toolName}
                <p class="text-sm opacity-70">Tool: {message.content.toolName}</p>
              {/if}
            </div>
          </div>
        {/each}
      {:else}
        <div class="h-full flex items-center justify-center text-muted-foreground">
          Select or create a session to start
        </div>
      {/if}
    </div>

    <!-- Input Area -->
    {#if selectedSession}
      <div class="border-t border-border p-4">
        <form onsubmit={sendMessage} class="flex gap-2">
          <input
            type="text"
            bind:value={inputText}
            disabled={isLoading}
            placeholder="Type a message..."
            class="flex-1 px-4 py-2 border border-input bg-background rounded-md focus:outline-none focus:ring-2 focus:ring-ring"
          />
          <button
            type="submit"
            disabled={isLoading || !inputText.trim()}
            class="px-6 py-2 bg-primary text-primary-foreground rounded-md hover:bg-primary/90 disabled:opacity-50 disabled:cursor-not-allowed"
          >
            {isLoading ? "Sending..." : "Send"}
          </button>
        </form>
      </div>
    {/if}
  </div>
</main>
