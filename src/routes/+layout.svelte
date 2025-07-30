<script lang="ts">
  import "../app.css";
  import { page } from "$app/stores";
  import { onMount } from "svelte";
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
    Plus,
    PlayCircle,
    PauseCircle,
    CheckCircle,
    XCircle,
    Sun,
    Moon,
  } from "lucide-svelte";
  import { goto } from "$app/navigation";
  import type { Snippet } from "svelte";
  import { sessions, runningSessions } from "$lib/stores/session";
  import { projects, projectsMap } from "$lib/stores/project";
  import { theme, type Theme } from "$lib/stores/theme";
  import type { SessionStatus } from "$lib/types";

  let { children }: { children?: Snippet } = $props();

  // State
  let sidebarCollapsed = $state(false);
  let currentTheme = $state<Theme>("system");

  // Load data on mount
  onMount(async () => {
    // Initialize theme
    theme.init();

    // Subscribe to theme changes
    theme.subscribe((value) => {
      currentTheme = value;
    });

    try {
      await Promise.all([sessions.load(), projects.load()]);
    } catch (error) {
      console.error("Failed to load initial data:", error);
    }
  });

  // Computed values
  const recentSessions = $derived(
    $sessions
      .filter((s) => s.status.type !== "running")
      .slice(0, 5)
      .map((s) => {
        return {
          ...s,
        };
      }),
  );

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

  // Breadcrumb items - context-aware
  const breadcrumbItems = $derived(
    (() => {
      const basePath = currentPath.split("/")[1];

      // For session detail pages, show actual context
      if (currentPath.startsWith("/session/")) {
        const sessionId = currentPath.split("/")[2];
        const session = $sessions.find((s) => s.id === sessionId);
        if (session) {
          return [{ label: session.title }];
        }
      }

      // For other pages, just show the page name
      return [{ label: getViewName(currentPath) }];
    })(),
  );

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

  function getSessionIcon(status: SessionStatus) {
    switch (status.type) {
      case "running":
        return PlayCircle;
      case "exited":
        return CheckCircle;
      case "failed":
        return XCircle;
      default:
        return MessageSquare;
    }
  }

  function getSessionIconColor(status: SessionStatus) {
    switch (status.type) {
      case "running":
        return "text-green-500";
      case "exited":
        return "text-muted-foreground";
      case "failed":
        return "text-red-500";
      default:
        return "text-muted-foreground";
    }
  }

  function toggleTheme() {
    const themes: Theme[] = ["light", "dark", "system"];
    const currentIndex = themes.indexOf(currentTheme as Theme);
    const nextIndex = (currentIndex + 1) % themes.length;
    theme.set(themes[nextIndex]);
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

    <!-- Theme Toggle at the bottom -->
    <div class="mt-auto mb-2">
      <ActivityBarItem separator />
      <ActivityBarItem
        icon={currentTheme === "dark" ? Sun : Moon}
        label={`Switch to ${currentTheme === "dark" ? "light" : currentTheme === "light" ? "system" : "dark"} mode`}
        onclick={toggleTheme}
      />
    </div>
  </ActivityBar>

  <!-- Side Panel -->
  <SidePanel bind:collapsed={sidebarCollapsed}>
    <!-- Active Sessions -->
    <SidePanelSection title="Active Sessions">
      {#if $runningSessions.length === 0}
        <div class="py-3 text-center text-sm text-muted-foreground">No active sessions</div>
      {:else}
        {#each $runningSessions as session}
          <Button
            variant="ghost"
            size="sm"
            class="mb-3 h-auto w-full justify-start p-3"
            onclick={() => navigateTo(`/session/${session.id}`)}
          >
            <div class="flex w-full items-start gap-3">
              <PlayCircle class="mt-0.5 h-4 w-4 shrink-0 text-green-500" />
              <div class="flex-1 overflow-hidden text-left">
                <div class="truncate font-medium">{session.title}</div>
              </div>
            </div>
          </Button>
        {/each}
      {/if}
    </SidePanelSection>

    <!-- Recent Sessions -->
    <SidePanelSection title="Recent Sessions" class="flex-1 overflow-y-auto">
      {#if recentSessions.length === 0}
        <div class="py-3 text-center text-sm text-muted-foreground">No recent sessions</div>
      {:else}
        {#each recentSessions as session}
          <Button
            variant="ghost"
            size="sm"
            class="mb-3 h-auto w-full justify-start p-3"
            onclick={() => navigateTo(`/session/${session.id}`)}
          >
            <div class="flex w-full items-start gap-3">
              {#if session.status.type === "exited"}
                <CheckCircle class="mt-0.5 h-4 w-4 shrink-0 text-muted-foreground" />
              {:else if session.status.type === "failed"}
                <XCircle class="mt-0.5 h-4 w-4 shrink-0 text-red-500" />
              {:else}
                <PauseCircle class="mt-0.5 h-4 w-4 shrink-0 text-yellow-500" />
              {/if}
              <div class="flex-1 overflow-hidden text-left">
                <div class="truncate">{session.title}</div>
              </div>
            </div>
          </Button>
        {/each}
      {/if}
    </SidePanelSection>

    <!-- Quick Actions -->
    <SidePanelSection title="Quick Actions">
      <Button
        variant="outline"
        size="sm"
        class="mb-2 w-full gap-2"
        onclick={() => navigateTo("/sessions?action=new")}
      >
        <Plus class="h-4 w-4" />
        New Session
      </Button>
      <Button variant="ghost" size="sm" class="mb-2 w-full" onclick={() => navigateTo("/sessions")}>
        Browse All Sessions
      </Button>
      <Button variant="ghost" size="sm" class="w-full" onclick={() => navigateTo("/projects")}>
        Browse Projects
      </Button>
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
