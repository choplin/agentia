<script lang="ts">
  import { onMount } from "svelte";
  import type { MessageContent } from "$lib/claude";

  interface Props {
    content: MessageContent;
    isToolUse?: boolean;
  }

  let { content, isToolUse = true }: Props = $props();

  let isExpanded = $state(false);
  let isLongContent = $state(false);
  let contentElement = $state<HTMLElement>();

  const toolIcons: Record<string, string> = {
    // File operations
    Read: "📄",
    Write: "✏️",
    Edit: "📝",
    MultiEdit: "📝",

    // Search operations
    Grep: "🔍",
    Glob: "🔎",
    LS: "📁",

    // Terminal operations
    Bash: "💻",

    // Web operations
    WebFetch: "🌐",
    WebSearch: "🔍",

    // Task management
    TodoWrite: "✅",
    Task: "🤖",

    // Notebook operations
    NotebookRead: "📓",
    NotebookEdit: "📝",

    // Default
    default: "🔧",
  };

  const toolDescriptions: Record<string, string> = {
    Read: "Reading file",
    Write: "Writing file",
    Edit: "Editing file",
    MultiEdit: "Editing multiple sections",
    Grep: "Searching in files",
    Glob: "Finding files",
    LS: "Listing directory",
    Bash: "Running command",
    WebFetch: "Fetching webpage",
    WebSearch: "Searching the web",
    TodoWrite: "Updating tasks",
    Task: "Running agent task",
    NotebookRead: "Reading notebook",
    NotebookEdit: "Editing notebook",
  };

  function getToolIcon(toolName: string): string {
    return toolIcons[toolName] || toolIcons.default;
  }

  function getToolDescription(toolName: string): string {
    return toolDescriptions[toolName] || toolName;
  }

  function formatToolInput(input: any): string {
    if (typeof input === "string") return input;
    return JSON.stringify(input, null, 2);
  }

  onMount(() => {
    // Check if content is long enough to need expansion
    if (contentElement) {
      isLongContent = contentElement.scrollHeight > 150;
    }
  });
</script>

{#if isToolUse && content.toolName}
  <div class="tool-message">
    <div class="tool-header">
      <div class="flex items-center gap-2">
        <span class="tool-icon text-xl">{getToolIcon(content.toolName)}</span>
        <span class="font-semibold">{content.toolName}</span>
        {#if isToolUse}
          <span class="tool-description">- {getToolDescription(content.toolName)}</span>
          <span class="tool-status">
            <span class="status-dot"></span>
          </span>
        {/if}
      </div>
      {#if content.toolInput && isLongContent}
        <button
          onclick={() => (isExpanded = !isExpanded)}
          class="expand-button"
          aria-label={isExpanded ? "Collapse" : "Expand"}
        >
          <svg
            xmlns="http://www.w3.org/2000/svg"
            width="16"
            height="16"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
            stroke-linecap="round"
            stroke-linejoin="round"
            class="transform transition-transform {isExpanded ? 'rotate-180' : ''}"
          >
            <polyline points="6 9 12 15 18 9"></polyline>
          </svg>
        </button>
      {/if}
    </div>

    {#if content.toolInput}
      <div bind:this={contentElement} class="tool-content {isExpanded ? '' : 'collapsed'}">
        <pre class="tool-input">{formatToolInput(content.toolInput)}</pre>
      </div>
    {/if}
  </div>
{:else}
  <!-- Tool Result -->
  <div class="tool-result">
    <div class="result-header">
      <div class="flex items-center gap-2">
        <span class="result-icon">✓</span>
        <span class="text-sm font-medium">Result</span>
      </div>
    </div>
    {#if content.toolResult}
      <div class="result-content">
        <pre class="tool-output">{JSON.stringify(content.toolResult, null, 2)}</pre>
      </div>
    {:else if content.text}
      <div class="result-content">
        <p class="whitespace-pre-wrap text-sm">{content.text}</p>
      </div>
    {:else if content.error}
      <div class="result-content error">
        <p class="text-sm">❌ {content.error}</p>
      </div>
    {/if}
  </div>
{/if}

<style>
  .tool-message {
    background: rgb(30 41 59 / 0.3);
    border: 1px solid rgb(71 85 105 / 0.5);
    border-radius: 0.5rem;
    overflow: hidden;
  }

  .tool-header {
    padding: 0.75rem 1rem;
    background: rgb(30 41 59 / 0.3);
    border-bottom: 1px solid rgb(71 85 105 / 0.3);
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  .tool-icon {
    filter: grayscale(0.2);
  }

  .tool-description {
    font-size: 0.875rem;
    color: rgb(148 163 184);
    font-weight: normal;
  }

  .tool-status {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    margin-left: auto;
  }

  .status-dot {
    width: 8px;
    height: 8px;
    background: rgb(34 197 94);
    border-radius: 50%;
    animation: pulse 2s infinite;
  }

  @keyframes pulse {
    0%,
    100% {
      opacity: 1;
    }
    50% {
      opacity: 0.5;
    }
  }

  .expand-button {
    background: none;
    border: none;
    padding: 0.25rem;
    cursor: pointer;
    color: rgb(148 163 184);
    transition: color 0.2s;
  }

  .expand-button:hover {
    color: rgb(226 232 240);
  }

  .tool-content {
    max-height: 150px;
    overflow: hidden;
    transition: max-height 0.3s ease-out;
  }

  .tool-content.collapsed {
    position: relative;
  }

  .tool-content.collapsed::after {
    content: "";
    position: absolute;
    bottom: 0;
    left: 0;
    right: 0;
    height: 3rem;
    background: linear-gradient(to bottom, transparent, rgb(15 23 42 / 0.9));
    pointer-events: none;
  }

  .tool-content:not(.collapsed) {
    max-height: none;
  }

  .tool-input {
    padding: 1rem;
    margin: 0;
    font-size: 0.75rem;
    line-height: 1.5;
    color: rgb(226 232 240);
    background: rgb(15 23 42 / 0.5);
    overflow-x: auto;
    font-family: ui-monospace, SFMono-Regular, "SF Mono", Menlo, Monaco, Consolas, monospace;
  }

  .tool-result {
    background: rgb(30 41 59 / 0.2);
    border: 1px solid rgb(71 85 105 / 0.3);
    border-radius: 0.5rem;
    overflow: hidden;
  }

  .result-header {
    padding: 0.5rem 1rem;
    background: rgb(30 41 59 / 0.3);
    border-bottom: 1px solid rgb(71 85 105 / 0.3);
    color: rgb(148 163 184);
  }

  .result-icon {
    color: rgb(34 197 94);
    font-size: 0.875rem;
  }

  .result-content {
    padding: 0.75rem 1rem;
  }

  .result-content.error {
    color: rgb(239 68 68);
  }

  .tool-output {
    margin: 0;
    font-size: 0.75rem;
    line-height: 1.5;
    color: rgb(226 232 240);
    background: rgb(15 23 42 / 0.3);
    padding: 0.75rem;
    border-radius: 0.25rem;
    overflow-x: auto;
    font-family: ui-monospace, SFMono-Regular, "SF Mono", Menlo, Monaco, Consolas, monospace;
  }
</style>
