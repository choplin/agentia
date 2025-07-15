<script lang="ts">
  import "../app.css";
  import { page } from "$app/stores";
  import AppShell from "$lib/components/layout/AppShell.svelte";
  import ActivityBar from "$lib/components/layout/ActivityBar.svelte";
  import ActivityBarItem from "$lib/components/layout/ActivityBarItem.svelte";
  import SidePanel from "$lib/components/layout/SidePanel.svelte";
  import SidePanelSection from "$lib/components/layout/SidePanelSection.svelte";
  import MainContent from "$lib/components/layout/MainContent.svelte";
  import Breadcrumb from "$lib/components/layout/Breadcrumb.svelte";
  import { Button } from "$lib/components/ui/button";
  import * as Card from "$lib/components/ui/card";
  import {
    Home,
    MessageSquare,
    FolderOpen,
    GitBranch,
    Settings,
    Plug,
    BarChart3,
    Folder,
  } from "lucide-svelte";
  import { goto } from "$app/navigation";
  import type { Snippet } from "svelte";

  let { children }: { children?: Snippet } = $props();

  // State
  let sidebarCollapsed = $state(false);

  // Mock data
  const currentProject = {
    name: "agentia",
    path: "/Users/aki/workspace/agentia",
  };

  const worktrees = [
    { branch: "main", isDefault: true },
    { branch: "feature/ui-redesign" },
    { branch: "feature/payment" },
  ];

  const recentSessions = [
    { name: "Payment API Implementation", time: "2 hours ago", worktree: "feature/payment" },
    { name: "Bug Fix #123", time: "Yesterday", worktree: "fix/cart-bug" },
    { name: "Refactoring Discussion", time: "3 days ago", worktree: "main" },
  ];

  // Menu items
  type MenuItem = {
    id?: string;
    path?: string;
    icon?: any;
    label?: string;
    separator?: boolean;
  };

  const menuItems: MenuItem[] = [
    { id: "home", path: "/", icon: Home, label: "Home" },
    { id: "sessions", path: "/sessions", icon: MessageSquare, label: "Sessions" },
    { id: "projects", path: "/projects", icon: FolderOpen, label: "Projects" },
    { id: "worktrees", path: "/worktrees", icon: GitBranch, label: "Worktrees" },
    { separator: true },
    { id: "settings", path: "/settings", icon: Settings, label: "Settings" },
    { id: "mcp", path: "/mcp", icon: Plug, label: "MCP Servers" },
    { id: "stats", path: "/stats", icon: BarChart3, label: "Statistics" },
  ];

  // Get current path to determine active menu item
  const currentPath = $derived($page.url.pathname);

  // Breadcrumb items
  const breadcrumbItems = $derived([
    { label: "Project" },
    { label: currentProject?.name || "No Selection" },
    { label: getViewName(currentPath) },
  ]);

  function getViewName(path: string): string {
    const names: Record<string, string> = {
      "/": "Home",
      "/sessions": "Sessions",
      "/projects": "Projects",
      "/worktrees": "Worktrees",
      "/settings": "Settings",
      "/mcp": "MCP Servers",
      "/stats": "Statistics",
    };
    // Handle dynamic routes like /session/[id]
    if (path.startsWith("/session/")) {
      return "Session Details";
    }
    return names[path] || "";
  }

  function navigateTo(path: string) {
    goto(path);
  }
</script>

<AppShell>
  <!-- Activity Bar -->
  <ActivityBar>
    {#each menuItems as item}
      {#if item.separator}
        <ActivityBarItem separator />
      {:else}
        <ActivityBarItem
          icon={item.icon}
          label={item.label}
          active={currentPath === item.path}
          onclick={() => item.path && navigateTo(item.path)}
        />
      {/if}
    {/each}
  </ActivityBar>

  <!-- Side Panel -->
  <SidePanel bind:collapsed={sidebarCollapsed}>
    <!-- Current Project -->
    <SidePanelSection title="Current Project">
      <Card.Root class="border-muted shadow-none">
        <Card.Content class="p-3">
          <div class="flex items-center gap-2 font-semibold">
            <Folder class="h-4 w-4" />
            {currentProject?.name || "No Selection"}
          </div>
          <div class="text-xs text-muted-foreground">
            {currentProject?.path || "Please select a project"}
          </div>
        </Card.Content>
      </Card.Root>
      <Button
        variant="outline"
        size="sm"
        class="mt-2 w-full"
        onclick={() => navigateTo("/projects")}
      >
        Change Project
      </Button>
    </SidePanelSection>

    <!-- Worktrees -->
    <SidePanelSection title="Worktrees">
      {#each worktrees as worktree}
        <Button variant="ghost" size="sm" class="w-full justify-start gap-2">
          <GitBranch class="h-4 w-4" />
          <span>
            {worktree.branch}
            {#if worktree.isDefault}
              <span class="ml-1 text-xs text-primary">(default)</span>
            {/if}
          </span>
        </Button>
      {/each}
      <Button
        variant="outline"
        size="sm"
        class="mt-2 w-full"
        onclick={() => navigateTo("/worktrees")}
      >
        Manage Worktrees
      </Button>
    </SidePanelSection>

    <!-- Recent Sessions -->
    <SidePanelSection title="Recent Sessions" class="flex-1 overflow-y-auto">
      {#each recentSessions as session}
        <Button variant="ghost" size="sm" class="mb-2 h-auto w-full justify-start p-2">
          <div class="text-left">
            <div>{session.name}</div>
            <div class="text-xs text-muted-foreground">
              {session.worktree} • {session.time}
            </div>
          </div>
        </Button>
      {/each}
    </SidePanelSection>

    <!-- Favorites -->
    <SidePanelSection title="Favorites">
      <div class="py-4 text-center text-sm text-muted-foreground">No favorites</div>
    </SidePanelSection>
  </SidePanel>

  <!-- Main Content -->
  <MainContent>
    <Breadcrumb items={breadcrumbItems} />
    <div class="flex-1 p-6">
      {@render children?.()}
    </div>
  </MainContent>
</AppShell>
