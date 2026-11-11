import { defineConfig } from "vitest/config";

// No `passWithNoTests`: an empty test run must fail, not pass silently.
export default defineConfig({
  test: {
    environment: "jsdom",
    include: ["src/**/*.test.ts"],
  },
});
