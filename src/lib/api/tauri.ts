import { invoke } from "@tauri-apps/api/core";
import type { Project, Session, SessionConfig } from "$lib/types";

// Project API
export async function getProjects(): Promise<Project[]> {
  return invoke<Project[]>("get_projects");
}

export async function createProject(name: string, path: string): Promise<Project> {
  return invoke<Project>("create_project", { name, path });
}

export async function selectProject(projectId: string): Promise<void> {
  return invoke("select_project", { projectId });
}

// Session API
export async function createSession(title: string, config: SessionConfig): Promise<Session> {
  return invoke<Session>("create_session", { title, config });
}

export async function getSession(sessionId: string): Promise<Session> {
  return invoke<Session>("get_session", { sessionId });
}

export async function listSessions(): Promise<Session[]> {
  return invoke<Session[]>("list_sessions");
}

export async function sendMessage(sessionId: string, message: string): Promise<void> {
  return invoke("send_message", { sessionId, message });
}

export async function stopSession(sessionId: string): Promise<void> {
  return invoke("stop_session", { sessionId });
}
