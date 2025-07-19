import { writable, derived } from "svelte/store";
import {
  listSessions,
  getSession,
  createSession as apiCreateSession,
  sendMessage as apiSendMessage,
  stopSession as apiStopSession,
} from "$lib/api/tauri";
import type { Session, SessionConfig } from "$lib/types";

// Sessions store
function createSessionsStore() {
  const { subscribe, set, update } = writable<Session[]>([]);

  return {
    subscribe,
    update,
    async load() {
      try {
        const sessionsList = await listSessions();
        set(sessionsList);
        return sessionsList;
      } catch (error) {
        console.error("Failed to load sessions:", error);
        throw error;
      }
    },
    async create(title: string, config: SessionConfig) {
      try {
        const session = await apiCreateSession(title, config);
        update((sessions) => [session, ...sessions]);
        return session;
      } catch (error) {
        console.error("Failed to create session:", error);
        throw error;
      }
    },
    async get(sessionId: string) {
      try {
        const session = await getSession(sessionId);
        // Update the session in the store
        update((sessions) => {
          const index = sessions.findIndex((s) => s.id === sessionId);
          if (index !== -1) {
            sessions[index] = session;
          }
          return sessions;
        });
        return session;
      } catch (error) {
        console.error("Failed to get session:", error);
        throw error;
      }
    },
    async sendMessage(sessionId: string, message: string) {
      try {
        await apiSendMessage(sessionId, message);
      } catch (error) {
        console.error("Failed to send message:", error);
        throw error;
      }
    },
    async stop(sessionId: string) {
      try {
        await apiStopSession(sessionId);
        // Update session status in store
        update((sessions) => {
          const session = sessions.find((s) => s.id === sessionId);
          if (session) {
            session.status = { type: "exited" };
          }
          return sessions;
        });
      } catch (error) {
        console.error("Failed to stop session:", error);
        throw error;
      }
    },
  };
}

// Current session store
function createCurrentSessionStore() {
  const { subscribe, set } = writable<Session | null>(null);

  return {
    subscribe,
    set,
    clear: () => set(null),
  };
}

export const sessions = createSessionsStore();
export const currentSession = createCurrentSessionStore();

// Derived store for running sessions
export const runningSessions = derived(sessions, ($sessions) =>
  $sessions.filter((s) => s.status.type === "running"),
);

// Derived store for quick session lookup
export const sessionsMap = derived(sessions, ($sessions) => {
  return new Map($sessions.map((s) => [s.id, s]));
});
