<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";

  let name = $state("");
  let greetMsg = $state("");

  async function greet(event: Event) {
    event.preventDefault();
    greetMsg = await invoke("greet", { name });
  }
</script>

<main class="min-h-screen bg-background text-foreground flex items-center justify-center">
  <div class="max-w-2xl mx-auto p-8 text-center">
    <h1 class="text-4xl font-bold mb-8">Welcome to Agentia</h1>

    <div class="flex justify-center gap-4 mb-8">
      <div
        class="w-24 h-24 bg-slate-200 dark:bg-slate-800 rounded-lg flex items-center justify-center"
      >
        <span class="text-3xl">🤖</span>
      </div>
    </div>

    <p class="text-muted-foreground mb-8">AI-powered development environment</p>

    <form class="flex gap-2 justify-center" onsubmit={greet}>
      <input
        class="px-4 py-2 border border-input bg-background rounded-md"
        placeholder="Enter a name..."
        bind:value={name}
      />
      <button
        type="submit"
        class="px-4 py-2 bg-primary text-primary-foreground rounded-md hover:bg-primary/90"
      >
        Greet
      </button>
    </form>

    {#if greetMsg}
      <p class="mt-4 text-lg">{greetMsg}</p>
    {/if}
  </div>
</main>
