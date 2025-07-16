<script lang="ts" module>
  import { cn } from "$lib/utils.js";
  import type { Snippet } from "svelte";

  export interface TooltipProps {
    children: Snippet;
    class?: string;
    delayDuration?: number;
    open?: boolean;
  }
</script>

<script lang="ts">
  let { children, class: className, delayDuration = 700, open = false }: TooltipProps = $props();

  let isOpen = $state(open);
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
</div>
