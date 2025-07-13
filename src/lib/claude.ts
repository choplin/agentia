import { invoke } from "@tauri-apps/api/core";
import { listen, type UnlistenFn } from "@tauri-apps/api/event";

export enum MessageRole {
  User = "user",
  Assistant = "assistant",
}

export enum MessageType {
  Text = "text",
  ToolUse = "toolUse",
  ToolResult = "toolResult",
  Error = "error",
}

export enum SessionStatus {
  Active = "active",
  Paused = "paused",
  Completed = "completed",
  Failed = "failed",
}

export interface MessageContent {
  text?: string;
  toolName?: string;
  toolInput?: any;
  toolResult?: any;
  error?: string;
}

export interface Message {
  id: string;
  role: MessageRole;
  type: MessageType; // Rust側では "type" として送信される
  content: MessageContent;
  timestamp: string;
}

export interface SessionConfig {
  model: string;
  temperature?: number;
  maxTokens?: number;
  permissionMode: string;
  workingDirectory?: string;
}

export interface Session {
  id: string;
  title: string;
  config: SessionConfig;
  messages: Message[];
  status: SessionStatus;
  createdAt: string;
  updatedAt: string;
}

export interface Project {
  id: string;
  path: string;
  name: string;
  createdAt: string;
  updatedAt: string;
  settings: string | null;
}

export class ClaudeAPI {
  private listeners: Map<string, UnlistenFn> = new Map();

  async createSession(title: string, config?: Partial<SessionConfig>): Promise<Session> {
    const defaultConfig: SessionConfig = {
      model: "claude-3-5-sonnet-20241022",
      permissionMode: "default",
      ...config,
    };

    return invoke<Session>("create_session", { title, config: defaultConfig });
  }

  async getSession(sessionId: string): Promise<Session> {
    return invoke<Session>("get_session", { sessionId });
  }

  async listSessions(): Promise<Session[]> {
    return invoke<Session[]>("list_sessions");
  }

  async sendMessage(sessionId: string, message: string): Promise<void> {
    return invoke<void>("send_message", { sessionId, message });
  }

  async stopSession(sessionId: string): Promise<void> {
    return invoke<void>("stop_session", { sessionId });
  }

  async listenToSession(
    sessionId: string,
    callback: (message: Message) => void,
  ): Promise<() => void> {
    const eventName = `session-${sessionId}-message`;

    // Remove existing listener if any
    const existingListener = this.listeners.get(eventName);
    if (existingListener) {
      existingListener();
    }

    // Create new listener
    const unlisten = await listen<Message>(eventName, (event) => {
      callback(event.payload);
    });

    this.listeners.set(eventName, unlisten);

    // Return cleanup function
    return () => {
      unlisten();
      this.listeners.delete(eventName);
    };
  }

  async getProjects(): Promise<Project[]> {
    return invoke<Project[]>("get_projects");
  }

  async selectProject(projectId: string): Promise<void> {
    return invoke<void>("select_project", { projectId });
  }

  async createProject(name: string, path: string): Promise<Project> {
    return invoke<Project>("create_project", { name, path });
  }

  cleanup() {
    // Clean up all listeners
    for (const unlisten of this.listeners.values()) {
      unlisten();
    }
    this.listeners.clear();
  }
}

export const claudeAPI = new ClaudeAPI();
