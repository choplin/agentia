<script lang="ts">
  import { Button } from "$lib/components/ui/button";
  import * as Card from "$lib/components/ui/card";
  import { Save } from "lucide-svelte";

  // Mock settings - will be fetched from API later
  let settings = $state({
    general: {
      theme: "dark",
      language: "en",
      autoSave: true,
    },
    editor: {
      fontSize: 14,
      tabSize: 2,
      wordWrap: true,
    },
    api: {
      claudeApiKey: "sk-...",
      maxTokens: 4096,
    },
  });

  function saveSettings() {
    console.log("Save settings:", settings);
  }
</script>

<div class="space-y-6">
  <div class="flex items-center justify-between">
    <h1 class="text-3xl font-bold">Settings</h1>
    <Button onclick={saveSettings}>
      <Save class="mr-2 h-4 w-4" />
      Save
    </Button>
  </div>

  <!-- General Settings -->
  <Card.Root>
    <Card.Header>
      <Card.Title>General Settings</Card.Title>
      <Card.Description>Basic application settings</Card.Description>
    </Card.Header>
    <Card.Content class="space-y-4">
      <div>
        <label for="theme" class="text-sm font-medium">Theme</label>
        <select
          id="theme"
          class="mt-1 w-full rounded-md border border-input bg-background px-3 py-2 text-sm"
          bind:value={settings.general.theme}
        >
          <option value="light">Light</option>
          <option value="dark">Dark</option>
          <option value="system">System</option>
        </select>
      </div>
      <div>
        <label for="language" class="text-sm font-medium">Language</label>
        <select
          id="language"
          class="mt-1 w-full rounded-md border border-input bg-background px-3 py-2 text-sm"
          bind:value={settings.general.language}
        >
          <option value="ja">Japanese</option>
          <option value="en">English</option>
        </select>
      </div>
      <div class="flex items-center gap-2">
        <input type="checkbox" id="autoSave" bind:checked={settings.general.autoSave} />
        <label for="autoSave" class="text-sm">Enable auto-save</label>
      </div>
    </Card.Content>
  </Card.Root>

  <!-- Editor Settings -->
  <Card.Root>
    <Card.Header>
      <Card.Title>Editor Settings</Card.Title>
      <Card.Description>Code editor settings</Card.Description>
    </Card.Header>
    <Card.Content class="space-y-4">
      <div>
        <label for="fontSize" class="text-sm font-medium">Font Size</label>
        <input
          id="fontSize"
          type="number"
          class="mt-1 w-full rounded-md border border-input bg-background px-3 py-2 text-sm"
          bind:value={settings.editor.fontSize}
          min="10"
          max="24"
        />
      </div>
      <div>
        <label for="tabSize" class="text-sm font-medium">Tab Size</label>
        <input
          id="tabSize"
          type="number"
          class="mt-1 w-full rounded-md border border-input bg-background px-3 py-2 text-sm"
          bind:value={settings.editor.tabSize}
          min="2"
          max="8"
        />
      </div>
      <div class="flex items-center gap-2">
        <input type="checkbox" id="wordWrap" bind:checked={settings.editor.wordWrap} />
        <label for="wordWrap" class="text-sm">Enable word wrap</label>
      </div>
    </Card.Content>
  </Card.Root>

  <!-- API Settings -->
  <Card.Root>
    <Card.Header>
      <Card.Title>API Settings</Card.Title>
      <Card.Description>Claude API configuration</Card.Description>
    </Card.Header>
    <Card.Content class="space-y-4">
      <div>
        <label for="apiKey" class="text-sm font-medium">API Key</label>
        <input
          id="apiKey"
          type="password"
          class="mt-1 w-full rounded-md border border-input bg-background px-3 py-2 text-sm"
          bind:value={settings.api.claudeApiKey}
        />
      </div>
      <div>
        <label for="maxTokens" class="text-sm font-medium">Max Tokens</label>
        <input
          id="maxTokens"
          type="number"
          class="mt-1 w-full rounded-md border border-input bg-background px-3 py-2 text-sm"
          bind:value={settings.api.maxTokens}
          min="1000"
          max="10000"
        />
      </div>
    </Card.Content>
  </Card.Root>
</div>
