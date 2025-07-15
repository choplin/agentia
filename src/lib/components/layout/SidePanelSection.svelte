<script lang="ts">
  import { cn } from "$lib/utils";
  import type { Snippet } from "svelte";

  interface Props {
    title?: string;
    class?: string;
    children?: Snippet;
  }

  let { title, class: className, children }: Props = $props();
</script>

<div class={cn("border-b p-4", className)}>
  {#if title}
    <h3 class="mb-2 text-xs font-medium uppercase tracking-wider text-muted-foreground">
      {title}
    </h3>
  {/if}
  {#if children}
    {@render children()}
  {/if}
</div>
