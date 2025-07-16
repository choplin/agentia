<script lang="ts">
  import { Button } from "$lib/components/ui/button";
  import * as Card from "$lib/components/ui/card";
  import * as Select from "$lib/components/ui/select";
  import { Input } from "$lib/components/ui/input";
  import { Checkbox } from "$lib/components/ui/checkbox";
  import { Save, Sun, Moon, Monitor } from "lucide-svelte";
  import { theme as themeStore, type Theme } from "$lib/stores/theme";
  import { onMount } from "svelte";

  // Get current theme
  let currentTheme = $state<Theme>("system");
  onMount(() => {
    themeStore.subscribe((value) => {
      currentTheme = value;
    });
  });

  // Mock settings - will be fetched from API later
  let settings = $state({
    general: {
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
        <div class="text-sm font-medium mb-2">Theme</div>
        <Select.Root
          type="single"
          value={currentTheme}
          onValueChange={(value) => {
            if (value) {
              currentTheme = value as Theme;
              themeStore.set(currentTheme);
            }
          }}
        >
          <Select.Trigger class="w-full">
            <div class="flex items-center gap-2">
              {#if currentTheme === "light"}
                <Sun class="h-4 w-4" />
                <span>Light</span>
              {:else if currentTheme === "dark"}
                <Moon class="h-4 w-4" />
                <span>Dark</span>
              {:else}
                <Monitor class="h-4 w-4" />
                <span>System</span>
              {/if}
            </div>
          </Select.Trigger>
          <Select.Content>
            <Select.Item value="light">
              <div class="flex items-center gap-2">
                <Sun class="h-4 w-4" />
                <span>Light</span>
              </div>
            </Select.Item>
            <Select.Item value="dark">
              <div class="flex items-center gap-2">
                <Moon class="h-4 w-4" />
                <span>Dark</span>
              </div>
            </Select.Item>
            <Select.Item value="system">
              <div class="flex items-center gap-2">
                <Monitor class="h-4 w-4" />
                <span>System</span>
              </div>
            </Select.Item>
          </Select.Content>
        </Select.Root>
      </div>
      <div>
        <div class="text-sm font-medium mb-2">Language</div>
        <Select.Root
          type="single"
          value={settings.general.language}
          onValueChange={(value) => {
            if (value) {
              settings.general.language = value;
            }
          }}
        >
          <Select.Trigger class="w-full">
            <span>{settings.general.language === "ja" ? "Japanese" : "English"}</span>
          </Select.Trigger>
          <Select.Content>
            <Select.Item value="ja">Japanese</Select.Item>
            <Select.Item value="en">English</Select.Item>
          </Select.Content>
        </Select.Root>
      </div>
      <div class="flex items-center gap-2">
        <Checkbox
          id="autoSave"
          checked={settings.general.autoSave}
          onCheckedChange={(checked) => (settings.general.autoSave = !!checked)}
        />
        <label for="autoSave" class="text-sm cursor-pointer">Enable auto-save</label>
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
        <Input
          id="fontSize"
          type="number"
          class="mt-1"
          bind:value={settings.editor.fontSize}
          min="10"
          max="24"
        />
      </div>
      <div>
        <label for="tabSize" class="text-sm font-medium">Tab Size</label>
        <Input
          id="tabSize"
          type="number"
          class="mt-1"
          bind:value={settings.editor.tabSize}
          min="2"
          max="8"
        />
      </div>
      <div class="flex items-center gap-2">
        <Checkbox
          id="wordWrap"
          checked={settings.editor.wordWrap}
          onCheckedChange={(checked) => (settings.editor.wordWrap = !!checked)}
        />
        <label for="wordWrap" class="text-sm cursor-pointer">Enable word wrap</label>
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
        <Input id="apiKey" type="password" class="mt-1" bind:value={settings.api.claudeApiKey} />
      </div>
      <div>
        <label for="maxTokens" class="text-sm font-medium">Max Tokens</label>
        <Input
          id="maxTokens"
          type="number"
          class="mt-1"
          bind:value={settings.api.maxTokens}
          min="1000"
          max="10000"
        />
      </div>
    </Card.Content>
  </Card.Root>
</div>
