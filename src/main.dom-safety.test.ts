import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { strings } from "./strings";

// QA (REQ-013): main.ts must never parse markup. The error path shows fixed strings only, so a
// DOM-content assertion cannot tell textContent from innerHTML there. These tests therefore trap
// the HTML-parsing entry points of jsdom and fail if main.ts touches any of them while it renders.
const invokeMock = vi.hoisted(() => vi.fn());
vi.mock("@tauri-apps/api/core", () => ({ invoke: invokeMock }));

const XSS = "<img src=x onerror=alert(1)>";
const container = {
  id: "9bf105870a7d",
  name: XSS,
  image: "alpine:latest",
  state: { kind: "unknown", raw: XSS },
  status_text: XSS,
  scope: { id: "s", display_name: XSS },
};

const trapped: string[] = [];
const restores: Array<() => void> = [];

function trapSetter(proto: object, prop: string): void {
  const original = Object.getOwnPropertyDescriptor(proto, prop)!;
  Object.defineProperty(proto, prop, {
    ...original,
    set(this: Element, value: string) {
      trapped.push(`${prop}=${value}`);
      original.set!.call(this, value);
    },
  });
  restores.push(() => Object.defineProperty(proto, prop, original));
}

function trapMethod(proto: object, name: string): void {
  const original = (proto as Record<string, unknown>)[name] as (...a: unknown[]) => unknown;
  (proto as Record<string, unknown>)[name] = function (this: Element, ...args: unknown[]) {
    trapped.push(`${name}(${String(args[args.length - 1])})`);
    return original.apply(this, args);
  };
  restores.push(() => ((proto as Record<string, unknown>)[name] = original));
}

async function loadAppWithTraps(): Promise<HTMLElement> {
  vi.resetModules();
  document.body.innerHTML = '<main id="app"></main>'; // test setup, before the traps are armed
  trapSetter(Element.prototype, "innerHTML");
  trapSetter(Element.prototype, "outerHTML");
  trapMethod(Element.prototype, "insertAdjacentHTML");
  await import("./main");
  const root = document.getElementById("app")!;
  await vi.waitFor(() => expect(root.textContent).not.toBe(strings.loading));
  return root;
}

beforeEach(() => {
  trapped.length = 0;
  invokeMock.mockReset();
});

afterEach(() => {
  while (restores.length) restores.pop()!();
});

describe("main.ts never parses markup", () => {
  it("does not use innerHTML, outerHTML or insertAdjacentHTML when rendering a list", async () => {
    invokeMock.mockResolvedValue([container]);
    const root = await loadAppWithTraps();
    expect(root.querySelectorAll("tbody tr")).toHaveLength(1); // it really rendered
    expect(trapped).toEqual([]);
  });

  it("does not use innerHTML, outerHTML or insertAdjacentHTML on the error path", async () => {
    invokeMock.mockRejectedValue({ kind: "commandFailed", message: XSS });
    const root = await loadAppWithTraps();
    expect(root.textContent).toBe(strings.errors.commandFailed); // it really rendered the error
    expect(trapped).toEqual([]);
  });

  it("the trap itself works (guards against a silently dead trap)", async () => {
    trapSetter(Element.prototype, "innerHTML");
    document.createElement("div").innerHTML = "<b>x</b>";
    expect(trapped).toEqual(["innerHTML=<b>x</b>"]);
  });
});
