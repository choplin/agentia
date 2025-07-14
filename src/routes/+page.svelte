<script lang="ts">
  import { onMount } from "svelte";
  import { ClaudeAPI, MessageRole, MessageType } from "$lib/claude";
  import type { Session, Message, Project } from "$lib/claude";
  import ProjectSelector from "$lib/components/ProjectSelector.svelte";
  import CreateProjectDialog from "$lib/components/CreateProjectDialog.svelte";
  import MarkdownRenderer from "$lib/components/MarkdownRenderer.svelte";

  const api = new ClaudeAPI();

  let sessions = $state<Session[]>([]);
  let selectedSession = $state<Session | null>(null);
  let messages = $state<Message[]>([]);
  let inputText = $state("");
  let isLoading = $state(false);

  let projects = $state<Project[]>([]);
  let currentProject = $state<Project | null>(null);
  let showCreateProjectDialog = $state(false);

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
    });
  }

  async function sendMessage() {
    if (!selectedSession || !inputText.trim() || isLoading) return;

    const messageText = inputText.trim();
    isLoading = true;
    inputText = "";

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
    <div class="flex-1 overflow-y-auto p-4 space-y-4">
      {#if selectedSession}
        {#each messages as message, index}
          <div class="flex {message.role === MessageRole.User ? 'justify-end' : 'justify-start'}">
            <div
              class="max-w-[70%] rounded-lg px-4 py-2 {message.role === MessageRole.User
                ? 'bg-primary text-primary-foreground'
                : 'bg-muted'}"
            >
              {#if message.type === MessageType.Text && message.content.text}
                <MarkdownRenderer content={message.content.text} />
              {:else if message.type === MessageType.Error && message.content.error}
                <p class="text-destructive font-semibold">Error: {message.content.error}</p>
              {:else if message.type === MessageType.ToolUse && message.content.toolName}
                <div class="space-y-1">
                  <p class="text-sm font-semibold flex items-center gap-2">
                    <span class="inline-block w-4 h-4 bg-current rounded-full animate-pulse"></span>
                    Tool: {message.content.toolName}
                  </p>
                  {#if message.content.toolInput}
                    <pre class="text-xs bg-black/10 p-2 rounded overflow-x-auto">{JSON.stringify(
                        message.content.toolInput,
                        null,
                        2,
                      )}</pre>
                  {/if}
                </div>
              {:else if message.type === MessageType.ToolResult}
                <div class="text-sm">
                  <p class="font-semibold mb-1">Tool Result:</p>
                  {#if message.content.toolResult}
                    <pre class="text-xs bg-black/10 p-2 rounded overflow-x-auto">{JSON.stringify(
                        message.content.toolResult,
                        null,
                        2,
                      )}</pre>
                  {:else if message.content.text}
                    <MarkdownRenderer content={message.content.text} />
                  {/if}
                </div>
              {:else}
                <p class="text-yellow-500">Unknown message type or missing content</p>
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
