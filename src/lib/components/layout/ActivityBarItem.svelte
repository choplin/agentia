<script lang="ts">
  import { cn } from "$lib/utils";

  interface Props {
    icon?: string;
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
      "flex h-10 w-10 items-center justify-center rounded-md text-xl transition-colors",
      "hover:bg-accent hover:text-accent-foreground",
      "focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring",
      active && "bg-accent text-accent-foreground",
      className,
    )}
  >
    {icon}
  </button>
{/if}
