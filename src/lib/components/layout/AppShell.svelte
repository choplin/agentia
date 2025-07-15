<script lang="ts">
  import { cn } from "$lib/utils";

  interface Props {
    class?: string;
    children?: any;
  }

  let { class: className, children }: Props = $props();
</script>

<div class={cn("flex h-screen overflow-hidden bg-background text-foreground", className)}>
  {@render children?.()}
</div>
