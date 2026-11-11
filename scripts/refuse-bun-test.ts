// Loaded by `bun test` through bunfig.toml ([test] preload). `bun run test` runs Vitest,
// which never loads bunfig's [test] section, so this only fires on a mistaken `bun test`.
throw new Error(
  "Do not use `bun test`: it is bun's own runner and ignores vitest.config.ts (jsdom). Run `bun run test`.",
);
