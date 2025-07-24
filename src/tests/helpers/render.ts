/**
 * Render helper utilities for testing Svelte components
 */

import type { ComponentType } from "svelte";

export interface RenderResult {
  component: any;
  destroy: () => void;
}

export interface RenderOptions {
  props?: Record<string, any>;
  context?: Map<any, any>;
  target?: HTMLElement;
}

/**
 * Render a Svelte component with optional context and props
 * This is a basic implementation that will be enhanced in future tasks
 */
export function renderWithContext(Component: ComponentType, options?: RenderOptions): RenderResult {
  // Basic implementation - will be enhanced with Testing Library in future tasks
  const target = options?.target || document.body;
  const props = options?.props || {};

  const component = new Component({
    target,
    props,
  });

  return {
    component,
    destroy: () => component.$destroy(),
  };
}
