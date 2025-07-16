<script lang="ts" module>
  import { cn } from "$lib/utils.js";
  import type { Snippet } from "svelte";
  import type { HTMLAttributes } from "svelte/elements";

  export interface TooltipTriggerProps extends HTMLAttributes<HTMLDivElement> {
    children: Snippet;
    class?: string;
  }
</script>

<script lang="ts">
  let { children, class: className, ...restProps }: TooltipTriggerProps = $props();
</script>

<div class={cn("inline-flex", className)} aria-describedby="tooltip" {...restProps}>
  {@render children()}
</div>
