// Project types
export interface Project {
  id: number;
  path: string;
  name: string;
  createdAt: string;
  updatedAt: string;
  settings?: string;
}

// Worktree types
export interface Worktree {
  id: number;
  projectId: number;
  path: string;
  name: string;
  isMain: boolean;
  createdAt: string;
  updatedAt: string;
}

// Session types
export type SessionStatus =
  | { type: "running" }
  | { type: "exited" }
  | { type: "failed"; exitCode: number };

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
  claudeSessionId?: string;
}

// Claude CLI Session types
export interface ClaudeCliSession {
  id: number;
  sessionId: string;
  claudeSessionId: string;
  filePath: string;
  config: string;
  parentCliSessionId?: number;
  processPid?: number;
  exitCode?: number;
  startedAt: string;
  endedAt?: string;
}

export interface ActiveCliSession {
  sessionId: string;
  cliSessionId: number;
  startedAt: string;
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
