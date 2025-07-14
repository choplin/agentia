<script lang="ts">
  import { onMount, tick } from "svelte";
  import { ClaudeAPI, MessageRole, MessageType } from "$lib/claude";
  import type { Session, Message, Project } from "$lib/claude";
  import ProjectSelector from "$lib/components/ProjectSelector.svelte";
  import CreateProjectDialog from "$lib/components/CreateProjectDialog.svelte";
  import MarkdownRenderer from "$lib/components/MarkdownRenderer.svelte";
  import ToolMessage from "$lib/components/ToolMessage.svelte";

  const api = new ClaudeAPI();

  let sessions = $state<Session[]>([]);
  let selectedSession = $state<Session | null>(null);
  let messages = $state<Message[]>([]);
  let inputText = $state("");
  let isLoading = $state(false);

  let projects = $state<Project[]>([]);
  let currentProject = $state<Project | null>(null);
  let showCreateProjectDialog = $state(false);

  // Auto-scroll state
  let messagesContainer: HTMLDivElement;
  let shouldAutoScroll = $state(true);
  let isUserScrolling = false;
  let scrollTimeout: number | null = null;

  onMount(async () => {
    await loadProjects();
    await loadSessions();
  });

  async function loadProjects() {
    try {
      projects = await api.getProjects();
      if (projects.length > 0) {
        currentProject = projects[0];
      }
    } catch (error) {
      console.error("Failed to load projects:", error);
    }
  }

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
    console.log("Session messages:", messages);

    // Set up listener for new messages
    await api.listenToSession(session.id, (message) => {
      console.log("New message received:", JSON.stringify(message, null, 2));
      console.log("Current messages count:", messages.length);
      console.log(
        "Message role:",
        message.role,
        "vs User:",
        MessageRole.User,
        "vs Assistant:",
        MessageRole.Assistant,
      );
      console.log("Message type:", message.type, "vs Text:", MessageType.Text);
      messages = [...messages, message];
      console.log("Messages after update:", messages.length);

      // Auto-scroll to bottom on new message
      if (shouldAutoScroll) {
        tick().then(scrollToBottom);
      }
    });
  }

  async function sendMessage() {
    if (!selectedSession || !inputText.trim() || isLoading) return;

    const messageText = inputText.trim();
    isLoading = true;
    inputText = "";

    // Enable auto-scroll when sending a message
    shouldAutoScroll = true;

    try {
      await api.sendMessage(selectedSession.id, messageText);
    } catch (error) {
      console.error("Failed to send message:", error);
    } finally {
      isLoading = false;
    }
  }

  async function handleProjectSelect(project: Project) {
    try {
      await api.selectProject(project.id);
      currentProject = project;
      // Reload sessions for the new project
      selectedSession = null;
      messages = [];
      await loadSessions();
    } catch (error) {
      console.error("Failed to select project:", error);
    }
  }

  async function handleProjectCreated(project: Project) {
    projects = [...projects, project];
    currentProject = project;
    // Clear sessions for new project
    sessions = [];
    selectedSession = null;
    messages = [];
  }

  function scrollToBottom() {
    if (messagesContainer) {
      messagesContainer.scrollTo({
        top: messagesContainer.scrollHeight,
        behavior: "smooth",
      });
    }
  }

  function handleScroll() {
    if (!messagesContainer) return;

    // Clear existing timeout
    if (scrollTimeout) {
      clearTimeout(scrollTimeout);
    }

    // Set flag to indicate user is scrolling
    isUserScrolling = true;

    // Check if user is near bottom (within 100px)
    const isNearBottom =
      messagesContainer.scrollHeight -
        messagesContainer.scrollTop -
        messagesContainer.clientHeight <
      100;

    shouldAutoScroll = isNearBottom;

    // Reset user scrolling flag after a delay
    scrollTimeout = window.setTimeout(() => {
      isUserScrolling = false;
    }, 150);
  }

  // Scroll to bottom when selected session changes
  $effect(() => {
    if (selectedSession && messages.length > 0) {
      tick().then(scrollToBottom);
    }
  });
</script>

<main class="flex h-screen bg-background text-foreground">
  <!-- Sidebar -->
  <aside class="w-64 border-r border-border bg-card flex flex-col">
    <div class="p-4 border-b border-border">
      <ProjectSelector
        {projects}
        {currentProject}
        onProjectSelect={handleProjectSelect}
        onProjectCreate={() => (showCreateProjectDialog = true)}
      />
    </div>

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
          class="w-full px-4 py-3 text-left transition-colors relative {selectedSession?.id ===
          session.id
            ? 'bg-primary text-primary-foreground'
            : 'hover:bg-accent hover:text-accent-foreground'}"
        >
          {#if selectedSession?.id === session.id}
            <div class="absolute left-0 top-0 bottom-0 w-1 bg-primary-foreground"></div>
          {/if}
          <div class="font-medium {selectedSession?.id === session.id ? 'font-bold' : ''}">
            {session.title}
          </div>
          <div
            class="text-sm {selectedSession?.id === session.id
              ? 'text-primary-foreground/80'
              : 'text-muted-foreground'}"
          >
            {new Date(session.createdAt).toLocaleDateString()} • {new Date(
              session.createdAt,
            ).toLocaleTimeString()}
          </div>
        </button>
      {/each}
    </div>
  </aside>

  <!-- Main Content -->
  <div class="flex-1 flex flex-col">
    <!-- Session Header -->
    {#if selectedSession}
      <div class="border-b border-border p-4 bg-card">
        <h2 class="text-lg font-semibold">{selectedSession.title}</h2>
        <div class="text-sm text-muted-foreground">
          Created: {new Date(selectedSession.createdAt).toLocaleString()} • Messages: {messages.length}
        </div>
      </div>
    {/if}

    <!-- Messages Area -->
    <div
      bind:this={messagesContainer}
      onscroll={handleScroll}
      class="flex-1 overflow-y-auto p-4 space-y-4 relative"
    >
      {#if selectedSession}
        {#each messages as message, index}
          <div class="message-wrapper {message.role === MessageRole.User ? 'user' : 'assistant'}">
            <div class="flex {message.role === MessageRole.User ? 'justify-end' : 'justify-start'}">
              <div class="message-container">
                <div
                  class="message-bubble rounded-lg px-4 py-2 {message.role === MessageRole.User
                    ? 'bg-primary text-primary-foreground'
                    : 'bg-muted'}"
                >
                  {#if message.type === MessageType.Text && message.content.text}
                    <MarkdownRenderer content={message.content.text} />
                  {:else if message.type === MessageType.Error && message.content.error}
                    <p class="text-destructive font-semibold">Error: {message.content.error}</p>
                  {:else if message.type === MessageType.ToolUse}
                    <ToolMessage content={message.content} isToolUse={true} />
                  {:else if message.type === MessageType.ToolResult}
                    <ToolMessage content={message.content} isToolUse={false} />
                  {:else}
                    <p class="text-yellow-500">Unknown message type or missing content</p>
                  {/if}
                </div>
                {#if message.timestamp}
                  <div class="message-timestamp">
                    {new Date(message.timestamp).toLocaleTimeString()}
                  </div>
                {/if}
              </div>
            </div>
          </div>
        {/each}
      {:else}
        <div class="h-full flex items-center justify-center text-muted-foreground">
          Select or create a session to start
        </div>
      {/if}

      <!-- Scroll to bottom button -->
      {#if !shouldAutoScroll && messages.length > 0}
        <button
          onclick={() => {
            shouldAutoScroll = true;
            scrollToBottom();
          }}
          class="absolute bottom-4 right-4 bg-primary text-primary-foreground rounded-full p-3 shadow-lg hover:bg-primary/90 transition-all duration-200 flex items-center gap-2 animate-fade-in"
          aria-label="Scroll to bottom"
        >
          <svg
            xmlns="http://www.w3.org/2000/svg"
            width="20"
            height="20"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
            stroke-linecap="round"
            stroke-linejoin="round"
          >
            <path d="m7 13 5 5 5-5" />
            <path d="m7 6 5 5 5-5" />
          </svg>
        </button>
      {/if}
    </div>

    <!-- Input Area -->
    {#if selectedSession}
      <div class="border-t border-border p-4">
        <form
          onsubmit={(e) => {
            e.preventDefault();
            sendMessage();
          }}
          class="flex gap-2"
        >
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

<CreateProjectDialog
  isOpen={showCreateProjectDialog}
  onClose={() => (showCreateProjectDialog = false)}
  onProjectCreated={handleProjectCreated}
/>

<style>
  @keyframes fade-in {
    from {
      opacity: 0;
      transform: translateY(10px);
    }
    to {
      opacity: 1;
      transform: translateY(0);
    }
  }

  .animate-fade-in {
    animation: fade-in 0.3s ease-out;
  }

  .message-wrapper {
    margin-bottom: 0.75rem;
  }

  .message-wrapper:last-child {
    margin-bottom: 0;
  }

  .message-container {
    max-width: 70%;
  }

  .message-bubble {
    position: relative;
  }

  .message-timestamp {
    font-size: 0.75rem;
    color: rgb(148 163 184);
    margin-top: 0.25rem;
    padding: 0 0.25rem;
  }

  .message-wrapper.user .message-timestamp {
    text-align: right;
  }

  .message-wrapper.assistant .message-timestamp {
    text-align: left;
  }
</style>
