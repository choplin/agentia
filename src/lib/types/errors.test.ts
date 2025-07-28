import { describe, it, expect } from "vitest";
import { isAppError, getErrorDisplayMessage, type AppError } from "./errors";

describe("Error Type System", () => {
  describe("isAppError", () => {
    it("should identify valid AppError objects", () => {
      const validError: AppError = {
        type: "AlreadyExists",
        message: "Resource already exists",
      };
      expect(isAppError(validError)).toBe(true);
    });

    it("should reject invalid objects", () => {
      expect(isAppError(null)).toBe(false);
      expect(isAppError(undefined)).toBe(false);
      expect(isAppError("string error")).toBe(false);
      expect(isAppError({ type: "InvalidType" })).toBe(false);
      expect(isAppError({ message: "Missing type" })).toBe(false);
      expect(isAppError({ type: 123, message: "Invalid type" })).toBe(false);
    });

    it("should validate all AppError types", () => {
      const errorTypes = ["Database", "AlreadyExists", "InvalidInput", "Internal"];

      errorTypes.forEach((type) => {
        const error = { type, message: "Test message" };
        expect(isAppError(error)).toBe(true);
      });
    });
  });

  describe("getErrorDisplayMessage", () => {
    it("should format AlreadyExists errors", () => {
      const error: AppError = {
        type: "AlreadyExists",
        message: "Directory already exists",
      };
      expect(getErrorDisplayMessage(error)).toBe("Already exists: Directory already exists");
    });

    it("should format InvalidInput errors", () => {
      const error: AppError = {
        type: "InvalidInput",
        message: "Name is required",
      };
      expect(getErrorDisplayMessage(error)).toBe("Invalid input: Name is required");
    });

    it("should provide generic message for Database errors", () => {
      const error: AppError = {
        type: "Database",
        message: "SQL error occurred",
      };
      expect(getErrorDisplayMessage(error)).toBe("Database error occurred. Please try again.");
    });

    it("should provide generic message for Internal errors", () => {
      const error: AppError = {
        type: "Internal",
        message: "Unexpected error",
      };
      expect(getErrorDisplayMessage(error)).toBe("An internal error occurred. Please try again.");
    });

    it("should handle non-AppError types", () => {
      expect(getErrorDisplayMessage("String error")).toBe("String error");
      expect(getErrorDisplayMessage(new Error("Error object"))).toBe("Error object");
    });

    it("should return default message for unknown types", () => {
      expect(getErrorDisplayMessage(null)).toBe("An unknown error occurred");
      expect(getErrorDisplayMessage(undefined)).toBe("An unknown error occurred");
      expect(getErrorDisplayMessage(123)).toBe("An unknown error occurred");
      expect(getErrorDisplayMessage({})).toBe("An unknown error occurred");
    });
  });

  describe("Integration with Tauri Mock", () => {
    it("should handle errors from TauriMock correctly", () => {
      // This simulates how errors come from the Tauri backend
      const tauriError = {
        type: "AlreadyExists",
        message: "Project already exists at path: /test/path",
      };

      expect(isAppError(tauriError)).toBe(true);
      expect(getErrorDisplayMessage(tauriError)).toBe(
        "Already exists: Project already exists at path: /test/path",
      );
    });
  });
});
