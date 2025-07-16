<script lang="ts">
  import { cn } from "$lib/utils";
  import { Button } from "$lib/components/ui/button";
  import { Separator } from "$lib/components/ui/separator";
  import { SimpleTooltip } from "$lib/components/ui/tooltip";

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
  <Separator class="my-2 w-8" />
{:else if label}
  <SimpleTooltip content={label} delayDuration={300} side="right">
    {#snippet children()}
      <Button {onclick} variant={active ? "secondary" : "ghost"} size="icon" class={className}>
        {#if icon}
          {@const Icon = icon}
          <Icon class="h-5 w-5" />
        {/if}
      </Button>
    {/snippet}
  </SimpleTooltip>
{:else}
  <Button {onclick} variant={active ? "secondary" : "ghost"} size="icon" class={className}>
    {#if icon}
      {@const Icon = icon}
      <Icon class="h-5 w-5" />
    {/if}
  </Button>
{/if}
