/**
 * Tauri API Mock System
 * Provides mock implementations for Tauri commands and events
 */

export type UnlistenFn = () => void;
export type EventCallback = (event: any) => void;
export type MockResponse = any | Error;

export class TauriMock {
  private mockCommands: Map<string, any> = new Map();
  private eventListeners: Map<string, Set<EventCallback>> = new Map();

  /**
   * Set a mock response for a specific command
   */
  setMockResponse(cmd: string, response: MockResponse): void {
    this.mockCommands.set(cmd, response);
  }

  /**
   * Mock a command with a specific response (alias for setMockResponse)
   */
  mockCommand(cmd: string, response: MockResponse): void {
    this.setMockResponse(cmd, response);
  }

  /**
   * Mock the invoke function for Tauri commands
   */
  async invoke(cmd: string, _args?: any): Promise<any> {
    if (!this.mockCommands.has(cmd)) {
      throw new Error(`Command ${cmd} is not mocked`);
    }
    return this.mockCommands.get(cmd);
  }

  /**
   * Mock the listen function for Tauri events
   */
  listen(event: string, handler: EventCallback): UnlistenFn {
    if (!this.eventListeners.has(event)) {
      this.eventListeners.set(event, new Set());
    }
    this.eventListeners.get(event)!.add(handler);

    return () => {
      const listeners = this.eventListeners.get(event);
      if (listeners) {
        listeners.delete(handler);
      }
    };
  }

  /**
   * Emit an event to all registered listeners
   */
  emit(event: string, payload: any): void {
    const listeners = this.eventListeners.get(event);
    if (listeners) {
      listeners.forEach((handler) => handler({ event, payload }));
    }
  }

  /**
   * Reset all mocks
   */
  reset(): void {
    this.mockCommands.clear();
    this.eventListeners.clear();
  }
}
