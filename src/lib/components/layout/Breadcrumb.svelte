<script lang="ts">
  import { cn } from "$lib/utils";
  import { ChevronRight } from "lucide-svelte";

  interface BreadcrumbItem {
    label: string;
    href?: string;
  }

  interface Props {
    items: BreadcrumbItem[];
    class?: string;
  }

  let { items, class: className }: Props = $props();
</script>

<nav class={cn("flex h-8 items-center border-b bg-background px-4 text-sm", className)}>
  {#each items as item, i}
    {#if i > 0}
      <ChevronRight class="mx-2 h-4 w-4 text-muted-foreground" />
    {/if}
    {#if item.href}
      <a
        href={item.href}
        class="hover:text-foreground {i === items.length - 1
          ? 'text-foreground'
          : 'text-muted-foreground'}"
      >
        {item.label}
      </a>
    {:else}
      <span class={i === items.length - 1 ? "text-foreground" : "text-muted-foreground"}>
        {item.label}
      </span>
    {/if}
  {/each}
</nav>
