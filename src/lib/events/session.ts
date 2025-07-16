import { listen, type Event, type UnlistenFn } from "@tauri-apps/api/event";
import type { Message } from "$lib/types";
import { sessions } from "$lib/stores/session";

export interface SessionMessageEvent {
  sessionId: string;
  message: Message;
}

// Listen to session messages
export async function listenToSessionMessages(sessionId: string): Promise<UnlistenFn> {
  return listen<Message>(`session-${sessionId}-message`, (event: Event<Message>) => {
    // Update session messages in the store
    sessions.update((sessionsList) => {
      const session = sessionsList.find((s) => s.id === sessionId);
      if (session) {
        session.messages = [...session.messages, event.payload];
        // Update session status based on message
        if (session.status === "completed" || session.status === "failed") {
          // Don't update status if already finished
        } else if (event.payload.role === "assistant") {
          // Session is active if assistant is responding
          session.status = "active";
        }
      }
      return sessionsList;
    });
  });
}

// Helper to manage multiple session listeners
export class SessionEventManager {
  private listeners: Map<string, UnlistenFn> = new Map();

  async subscribe(sessionId: string) {
    // Unsubscribe from existing listener if any
    await this.unsubscribe(sessionId);

    // Create new listener
    const unlisten = await listenToSessionMessages(sessionId);
    this.listeners.set(sessionId, unlisten);
  }

  async unsubscribe(sessionId: string) {
    const unlisten = this.listeners.get(sessionId);
    if (unlisten) {
      await unlisten();
      this.listeners.delete(sessionId);
    }
  }

  async unsubscribeAll() {
    for (const [_sessionId, unlisten] of this.listeners) {
      await unlisten();
    }
    this.listeners.clear();
  }
}
