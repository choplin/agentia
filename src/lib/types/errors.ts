/**
 * Error types for Tauri command handling
 */

/**
 * Structured error type from the Rust backend
 */
export type AppErrorType = "Database" | "AlreadyExists" | "InvalidInput" | "Internal";

export interface AppError {
  type: AppErrorType;
  message: string;
}

/**
 * Type guard to check if an error is an AppError
 */
export function isAppError(error: unknown): error is AppError {
  return (
    typeof error === "object" &&
    error !== null &&
    "type" in error &&
    "message" in error &&
    typeof (error as any).type === "string" &&
    typeof (error as any).message === "string"
  );
}

/**
 * Get user-friendly error message based on error type
 */
export function getErrorDisplayMessage(error: unknown): string {
  if (isAppError(error)) {
    switch (error.type) {
      case "AlreadyExists":
        return `Already exists: ${error.message}`;
      case "InvalidInput":
        return `Invalid input: ${error.message}`;
      case "Database":
        return "Database error occurred. Please try again.";
      case "Internal":
        return "An internal error occurred. Please try again.";
      default:
        return error.message;
    }
  }

  // For non-AppError types, handle directly
  if (typeof error === "string") {
    return error;
  }

  if (error instanceof Error) {
    return error.message;
  }

  return "An unknown error occurred";
}
