<script lang="ts">
  import { marked } from "marked";
  import { markedHighlight } from "marked-highlight";
  import hljs from "highlight.js";
  import { onMount, tick } from "svelte";

  export let content: string;

  let renderedHtml = "";
  let container: HTMLDivElement;

  // Configure marked with syntax highlighting
  marked.use(
    markedHighlight({
      langPrefix: "hljs language-",
      highlight(code, lang) {
        const language = hljs.getLanguage(lang) ? lang : "plaintext";
        return hljs.highlight(code, { language }).value;
      },
    }),
  );

  // Configure marked options
  marked.setOptions({
    gfm: true,
    breaks: true,
  });

  $: {
    // Render markdown to HTML
    renderedHtml = marked(content || "") as string;
    // Add copy buttons after render
    tick().then(addCopyButtons);
  }

  function addCopyButtons() {
    if (!container) return;

    const codeBlocks = container.querySelectorAll("pre");
    codeBlocks.forEach((pre) => {
      // Skip if already has copy button
      if (pre.querySelector(".copy-button")) return;

      const wrapper = document.createElement("div");
      wrapper.className = "code-block-wrapper";
      pre.parentNode?.insertBefore(wrapper, pre);
      wrapper.appendChild(pre);

      const button = document.createElement("button");
      button.className = "copy-button";
      button.textContent = "Copy";
      button.onclick = async () => {
        const code = pre.querySelector("code")?.textContent || "";
        await navigator.clipboard.writeText(code);
        button.textContent = "Copied!";
        setTimeout(() => {
          button.textContent = "Copy";
        }, 2000);
      };
      wrapper.appendChild(button);
    });
  }

  onMount(() => {
    // Import highlight.js styles
    import("highlight.js/styles/github-dark.min.css");
  });
</script>

<div bind:this={container} class="markdown-content prose prose-sm dark:prose-invert max-w-none">
  {@html renderedHtml}
</div>

<style>
  :global(.markdown-content) {
    /* Base styles */
    color: inherit;
    line-height: 1.6;
  }

  :global(.markdown-content p) {
    margin-bottom: 0.75rem;
  }

  :global(.markdown-content p:last-child) {
    margin-bottom: 0;
  }

  /* Code blocks */
  :global(.markdown-content pre) {
    background-color: rgb(13 18 28);
    border: 1px solid rgb(30 41 59);
    border-radius: 0.375rem;
    padding: 1rem;
    overflow-x: auto;
    margin-bottom: 0.75rem;
    font-size: 0.875rem;
  }

  :global(.markdown-content pre:last-child) {
    margin-bottom: 0;
  }

  :global(.markdown-content pre code) {
    background-color: transparent;
    padding: 0;
    border-radius: 0;
    font-size: 0.875rem;
  }

  /* Inline code */
  :global(.markdown-content code:not(pre code)) {
    background-color: rgb(30 41 59 / 0.5);
    padding: 0.125rem 0.375rem;
    border-radius: 0.25rem;
    font-size: 0.875rem;
    font-family:
      ui-monospace, SFMono-Regular, "SF Mono", Menlo, Monaco, Consolas, "Liberation Mono",
      "Courier New", monospace;
  }

  /* Headers */
  :global(.markdown-content h1),
  :global(.markdown-content h2),
  :global(.markdown-content h3),
  :global(.markdown-content h4),
  :global(.markdown-content h5),
  :global(.markdown-content h6) {
    font-weight: 600;
    margin-top: 1.5rem;
    margin-bottom: 0.75rem;
  }

  :global(.markdown-content h1) {
    font-size: 1.5rem;
  }

  :global(.markdown-content h2) {
    font-size: 1.25rem;
  }

  :global(.markdown-content h3) {
    font-size: 1.125rem;
  }

  /* Lists */
  :global(.markdown-content ul),
  :global(.markdown-content ol) {
    margin-bottom: 0.75rem;
    padding-left: 1.5rem;
  }

  :global(.markdown-content li) {
    margin-bottom: 0.25rem;
  }

  /* Links */
  :global(.markdown-content a) {
    color: rgb(59 130 246);
    text-decoration: underline;
  }

  :global(.markdown-content a:hover) {
    color: rgb(96 165 250);
  }

  /* Blockquotes */
  :global(.markdown-content blockquote) {
    border-left: 0.25rem solid rgb(75 85 99);
    padding-left: 1rem;
    margin-bottom: 0.75rem;
    font-style: italic;
    color: rgb(156 163 175);
  }

  /* Tables */
  :global(.markdown-content table) {
    width: 100%;
    border-collapse: collapse;
    margin-bottom: 0.75rem;
  }

  :global(.markdown-content th),
  :global(.markdown-content td) {
    border: 1px solid rgb(75 85 99);
    padding: 0.5rem;
    text-align: left;
  }

  :global(.markdown-content th) {
    background-color: rgb(30 41 59 / 0.5);
    font-weight: 600;
  }

  /* Horizontal rules */
  :global(.markdown-content hr) {
    border: none;
    border-top: 1px solid rgb(75 85 99);
    margin: 1.5rem 0;
  }

  /* Code block wrapper for copy button */
  :global(.code-block-wrapper) {
    position: relative;
    margin-bottom: 0.75rem;
  }

  :global(.code-block-wrapper pre) {
    margin-bottom: 0;
  }

  :global(.copy-button) {
    position: absolute;
    top: 0.5rem;
    right: 0.5rem;
    background-color: rgb(30 41 59);
    color: rgb(226 232 240);
    border: 1px solid rgb(71 85 105);
    padding: 0.25rem 0.75rem;
    border-radius: 0.25rem;
    font-size: 0.75rem;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.2s;
    opacity: 0.8;
  }

  :global(.copy-button:hover) {
    opacity: 1;
    background-color: rgb(51 65 85);
  }

  :global(.copy-button:active) {
    transform: scale(0.95);
  }
</style>
