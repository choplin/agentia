<script lang="ts" module>
  import { cn } from "$lib/utils.js";
  import type { Snippet } from "svelte";

  export interface SimpleTooltipProps {
    children: Snippet;
    content: string;
    class?: string;
    delayDuration?: number;
    side?: "top" | "right" | "bottom" | "left";
  }
</script>

<script lang="ts">
  let {
    children,
    content,
    class: className,
    delayDuration = 500,
    side = "top",
  }: SimpleTooltipProps = $props();

  let isOpen = $state(false);
  let timeoutId: ReturnType<typeof setTimeout> | undefined;

  function handleMouseEnter() {
    clearTimeout(timeoutId);
    timeoutId = setTimeout(() => {
      isOpen = true;
    }, delayDuration);
  }

  function handleMouseLeave() {
    clearTimeout(timeoutId);
    isOpen = false;
  }

  $effect(() => {
    return () => clearTimeout(timeoutId);
  });
</script>

<div
  class={cn("relative inline-flex", className)}
  onmouseenter={handleMouseEnter}
  onmouseleave={handleMouseLeave}
  role="button"
  tabindex="-1"
>
  {@render children()}

  {#if isOpen}
    <div
      role="tooltip"
      class={cn(
        "absolute z-50 animate-in fade-in-0 zoom-in-95",
        "rounded-md bg-popover px-3 py-1.5 text-xs text-popover-foreground shadow-md",
        "border border-border whitespace-nowrap",
        side === "top" && "-translate-x-1/2 left-1/2 bottom-full mb-2",
        side === "bottom" && "-translate-x-1/2 left-1/2 top-full mt-2",
        side === "left" && "-translate-y-1/2 top-1/2 right-full mr-2",
        side === "right" && "-translate-y-1/2 top-1/2 left-full ml-2",
      )}
    >
      {content}
      <div
        class={cn(
          "absolute h-2 w-2 rotate-45 bg-popover border-border",
          side === "top" && "-bottom-1 left-1/2 -translate-x-1/2 border-r border-b",
          side === "bottom" && "-top-1 left-1/2 -translate-x-1/2 border-l border-t",
          side === "left" && "-right-1 top-1/2 -translate-y-1/2 border-t border-r",
          side === "right" && "-left-1 top-1/2 -translate-y-1/2 border-b border-l",
        )}
      ></div>
    </div>
  {/if}
</div>
