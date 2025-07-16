<script lang="ts" module>
  import { cn } from "$lib/utils.js";
  import type { Snippet } from "svelte";
  import type { HTMLAttributes } from "svelte/elements";

  export interface TooltipContentProps extends HTMLAttributes<HTMLDivElement> {
    children: Snippet;
    class?: string;
    sideOffset?: number;
    open?: boolean;
  }
</script>

<script lang="ts">
  let {
    children,
    class: className,
    sideOffset = 4,
    open = false,
    ...restProps
  }: TooltipContentProps = $props();
</script>

{#if open}
  <div
    role="tooltip"
    class={cn(
      "absolute z-50 -translate-x-1/2 left-1/2 top-full animate-in fade-in-0 zoom-in-95 data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=closed]:zoom-out-95",
      "rounded-md bg-primary px-3 py-1.5 text-xs text-primary-foreground",
      className,
    )}
    style="margin-top: {sideOffset}px"
    {...restProps}
  >
    {@render children()}
  </div>
{/if}
