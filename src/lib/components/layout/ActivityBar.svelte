<script lang="ts">
  import { cn } from "$lib/utils";
  import type { Snippet } from "svelte";

  interface Props {
    class?: string;
    children?: Snippet;
  }

  let { class: className, children }: Props = $props();
</script>

<div class={cn("flex w-12 flex-col items-center gap-1 border-r bg-muted/10 py-2", className)}>
  {#if children}
    {@render children()}
  {/if}
</div>
