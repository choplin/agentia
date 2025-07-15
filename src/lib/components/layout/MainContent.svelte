<script lang="ts">
  import { cn } from "$lib/utils";
  import type { Snippet } from "svelte";

  interface Props {
    class?: string;
    children?: Snippet;
  }

  let { class: className, children }: Props = $props();
</script>

<div class={cn("flex flex-1 flex-col overflow-hidden bg-muted/5", className)}>
  {#if children}
    {@render children()}
  {/if}
</div>
