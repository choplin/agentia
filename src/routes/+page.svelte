<script lang="ts">
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

  // State
  let currentView = $state("sessions");
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
    { name: "決済API実装", time: "2時間前", worktree: "feature/payment" },
    { name: "バグ修正#123", time: "昨日", worktree: "fix/cart-bug" },
    { name: "リファクタリング検討", time: "3日前", worktree: "main" },
  ];

  // Menu items
  type MenuItem = {
    id?: string;
    icon?: any;
    label?: string;
    separator?: boolean;
  };

  const menuItems: MenuItem[] = [
    { id: "home", icon: Home, label: "ホーム" },
    { id: "sessions", icon: MessageSquare, label: "セッション一覧" },
    { id: "projects", icon: FolderOpen, label: "プロジェクト管理" },
    { id: "worktrees", icon: GitBranch, label: "Worktree管理" },
    { separator: true },
    { id: "settings", icon: Settings, label: "設定" },
    { id: "mcp", icon: Plug, label: "MCP管理" },
    { id: "stats", icon: BarChart3, label: "使用統計" },
  ];

  // Breadcrumb items
  const breadcrumbItems = $derived([
    { label: "プロジェクト" },
    { label: currentProject.name },
    { label: getViewName(currentView) },
  ]);

  function getViewName(view: string): string {
    const names: Record<string, string> = {
      home: "ホーム",
      sessions: "セッション一覧",
      projects: "プロジェクト管理",
      worktrees: "Worktree管理",
      settings: "設定",
      mcp: "MCP管理",
      stats: "使用統計",
    };
    return names[view] || "";
  }

  function setView(view: string) {
    currentView = view;
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
          active={currentView === item.id}
          onclick={() => item.id && setView(item.id)}
        />
      {/if}
    {/each}
  </ActivityBar>

  <!-- Side Panel -->
  <SidePanel bind:collapsed={sidebarCollapsed}>
    <!-- Current Project -->
    <SidePanelSection title="現在のプロジェクト">
      <Card.Root class="border-muted shadow-none">
        <Card.Content class="p-3">
          <div class="flex items-center gap-2 font-semibold">
            <Folder class="h-4 w-4" />
            {currentProject.name}
          </div>
          <div class="text-xs text-muted-foreground">
            {currentProject.path}
          </div>
        </Card.Content>
      </Card.Root>
      <Button variant="outline" size="sm" class="mt-2 w-full" onclick={() => setView("projects")}>
        プロジェクトを変更
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
      <Button variant="outline" size="sm" class="mt-2 w-full" onclick={() => setView("worktrees")}>
        Worktree管理
      </Button>
    </SidePanelSection>

    <!-- Recent Sessions -->
    <SidePanelSection title="最近のセッション" class="flex-1 overflow-y-auto">
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
    <SidePanelSection title="お気に入り">
      <div class="py-4 text-center text-sm text-muted-foreground">お気に入りはありません</div>
    </SidePanelSection>
  </SidePanel>

  <!-- Main Content -->
  <MainContent>
    <Breadcrumb items={breadcrumbItems} />

    <div class="flex-1 p-6">
      {#if currentView === "sessions"}
        <h2 class="mb-6 text-2xl font-semibold">セッション一覧</h2>
        <p class="text-muted-foreground">セッション一覧ページの内容がここに表示されます。</p>
      {:else if currentView === "projects"}
        <h2 class="mb-6 text-2xl font-semibold">プロジェクト管理</h2>
        <p class="text-muted-foreground">プロジェクト管理ページの内容がここに表示されます。</p>
      {:else if currentView === "worktrees"}
        <h2 class="mb-6 text-2xl font-semibold">Worktree管理</h2>
        <p class="text-muted-foreground">Worktree管理ページの内容がここに表示されます。</p>
      {:else if currentView === "home"}
        <h2 class="mb-6 text-2xl font-semibold">ホーム</h2>
        <p class="text-muted-foreground">ホームページの内容がここに表示されます。</p>
      {:else}
        <h2 class="mb-6 text-2xl font-semibold">{getViewName(currentView)}</h2>
        <p class="text-muted-foreground">このページは準備中です。</p>
      {/if}
    </div>
  </MainContent>
</AppShell>
