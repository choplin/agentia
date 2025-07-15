<script lang="ts">
  import { cn } from "$lib/utils";
  import type { Snippet } from "svelte";
  import { ChevronLeft, ChevronRight } from "lucide-svelte";

  interface Props {
    collapsed?: boolean;
    class?: string;
    children?: Snippet;
  }

  let { collapsed = $bindable(false), class: className, children }: Props = $props();

  function toggleCollapsed() {
    collapsed = !collapsed;
  }
</script>

<div
  class={cn(
    "relative flex flex-col border-r bg-background transition-all duration-300",
    collapsed ? "w-0 overflow-hidden" : "w-[260px]",
    className,
  )}
>
  {#if children}
    {@render children()}
  {/if}

  <button
    onclick={toggleCollapsed}
    class={cn(
      "absolute -right-3 top-1/2 z-10 flex h-12 w-6 -translate-y-1/2 items-center justify-center",
      "rounded-r border border-l-0 bg-background",
      "hover:bg-muted",
      "focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring",
    )}
  >
    {#if collapsed}
      <ChevronRight class="h-3 w-3" />
    {:else}
      <ChevronLeft class="h-3 w-3" />
    {/if}
  </button>
</div>
