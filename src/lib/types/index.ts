// Project types
export interface Project {
  id: string;
  path: string;
  name: string;
  createdAt: string;
  updatedAt: string;
  settings?: string;
}

// Session types
export type SessionStatus = "active" | "paused" | "completed" | "failed";

export interface SessionConfig {
  model: string;
  temperature?: number;
  maxTokens?: number;
  permissionMode: string;
  workingDirectory?: string;
}

export interface Session {
  id: string;
  projectId: string;
  title: string;
  config: SessionConfig;
  messages: Message[];
  status: SessionStatus;
  createdAt: string;
  updatedAt: string;
  claudeSessionId?: string;
}

// Message types
export type MessageRole = "user" | "assistant";

export type MessageType = "text" | "toolUse" | "toolResult" | "error";

export interface MessageContent {
  text?: string;
  toolName?: string;
  toolInput?: unknown;
  toolResult?: unknown;
  error?: string;
}

export interface Message {
  id: string;
  role: MessageRole;
  type: MessageType;
  content: MessageContent;
  timestamp: string;
}
