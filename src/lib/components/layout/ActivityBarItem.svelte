<script lang="ts">
  import { cn } from "$lib/utils";

  interface Props {
    icon?: any;
    label?: string;
    active?: boolean;
    separator?: boolean;
    onclick?: () => void;
    class?: string;
  }

  let {
    icon,
    label,
    active = false,
    separator = false,
    onclick,
    class: className,
  }: Props = $props();
</script>

{#if separator}
  <div class="my-2 h-px w-full bg-border"></div>
{:else}
  <button
    {onclick}
    title={label}
    class={cn(
      "flex h-10 w-10 items-center justify-center rounded transition-colors",
      "hover:bg-muted hover:text-foreground",
      "focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring",
      active && "bg-muted text-foreground",
      className,
    )}
  >
    {#if icon}
      {@const Icon = icon}
      <Icon class="h-5 w-5" />
    {/if}
  </button>
{/if}
