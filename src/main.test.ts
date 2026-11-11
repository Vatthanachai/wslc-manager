import { beforeEach, describe, expect, it, vi } from "vitest";
import { strings } from "./strings";

// The Tauri IPC layer is replaced by a test double: no Tauri, WebView2, Rust or WSL needed.
const invokeMock = vi.hoisted(() => vi.fn());
vi.mock("@tauri-apps/api/core", () => ({ invoke: invokeMock }));

const XSS = "<img src=x onerror=alert(1)>";

// Same shape as the one container the walking skeleton returns (FakeConnector).
const fakeContainer = {
  id: "9bf105870a7d",
  name: "aiyara-spike",
  image: "alpine:latest",
  state: { kind: "running" },
  status_text: "Up 3 seconds",
  scope: { id: "fake-session", display_name: "Fake session" },
};

/** Loads `main.ts` fresh (it runs on import) and waits until it has finished rendering. */
async function loadApp(): Promise<HTMLElement> {
  vi.resetModules();
  document.body.innerHTML = '<main id="app"></main>';
  await import("./main");
  const root = document.getElementById("app")!;
  await vi.waitFor(() => expect(root.textContent).not.toBe(strings.loading));
  return root;
}

function texts(nodes: ArrayLike<Element>): string[] {
  return Array.from(nodes, (n) => n.textContent ?? "");
}

beforeEach(() => {
  invokeMock.mockReset();
  document.title = "";
});

describe("container list", () => {
  it("shows the loading text while the command is pending", async () => {
    invokeMock.mockReturnValue(new Promise(() => {}));
    vi.resetModules();
    document.body.innerHTML = '<main id="app"></main>';
    await import("./main");
    expect(document.getElementById("app")!.textContent).toBe(strings.loading);
    expect(document.title).toBe(strings.appTitle);
  });

  it("renders one row with the fields of the one fake container", async () => {
    invokeMock.mockResolvedValue([fakeContainer]);
    const root = await loadApp();

    expect(invokeMock).toHaveBeenCalledWith("list_containers");
    expect(root.querySelector("h1")?.textContent).toBe(strings.containersHeading);
    const c = strings.columns;
    expect(texts(root.querySelectorAll("thead th"))).toEqual([
      c.name,
      c.image,
      c.state,
      c.status,
      c.session,
    ]);
    const rows = root.querySelectorAll("tbody tr");
    expect(rows).toHaveLength(1);
    expect(texts(rows[0]!.querySelectorAll("td"))).toEqual([
      fakeContainer.name,
      fakeContainer.image,
      strings.states.running,
      fakeContainer.status_text,
      fakeContainer.scope.display_name,
    ]);
    expect(root.querySelector("p")).toBeNull();
  });

  it("maps exited and unknown states and a missing status text", async () => {
    invokeMock.mockResolvedValue([
      { ...fakeContainer, state: { kind: "exited" }, status_text: null },
      { ...fakeContainer, state: { kind: "unknown", raw: "paused" } },
    ]);
    const root = await loadApp();
    const rows = Array.from(root.querySelectorAll("tbody tr"));
    expect(rows).toHaveLength(2);
    const [first, second] = rows.map((r) => texts(r.querySelectorAll("td")));
    expect(first![2]).toBe(strings.states.exited);
    expect(first![3]).toBe("");
    expect(second![2]).toBe(`${strings.states.unknown} (paused)`);
  });

  it("renders no data row and the empty-state text for an empty list", async () => {
    invokeMock.mockResolvedValue([]);
    const root = await loadApp();

    expect(root.querySelectorAll("tbody tr")).toHaveLength(0);
    expect(root.querySelector("table")).toBeNull();
    expect(root.querySelector("p")?.textContent).toBe(strings.empty);
  });

  it("shows container data from the engine literally, never as markup", async () => {
    invokeMock.mockResolvedValue([
      {
        ...fakeContainer,
        name: XSS,
        state: { kind: "unknown", raw: XSS },
        status_text: XSS,
      },
    ]);
    const root = await loadApp();

    expect(root.querySelector("img")).toBeNull();
    const cells = texts(root.querySelectorAll("tbody td"));
    expect(cells[0]).toBe(XSS);
    expect(cells[2]).toBe(`${strings.states.unknown} (${XSS})`);
    expect(cells[3]).toBe(XSS);
  });
});

describe("error path", () => {
  it.each([
    ["parseError", strings.errors.parseError],
    ["commandFailed", strings.errors.commandFailed],
    ["timeout", strings.errors.timeout],
    ["internal", strings.errors.internal],
  ])("shows the string-table text for error kind %s", async (kind, expected) => {
    invokeMock.mockRejectedValue({ kind });
    const root = await loadApp();
    expect(root.textContent).toBe(expected);
    expect(root.children).toHaveLength(0);
  });

  it.each([
    ["an unknown kind", { kind: "somethingNew" }],
    ["a plain string", "boom"],
    ["undefined", undefined],
  ])("falls back to the generic text for %s", async (_label, rejection) => {
    invokeMock.mockRejectedValue(rejection);
    const root = await loadApp();
    expect(root.textContent).toBe(strings.errors.generic);
  });

  it("never turns an error message into markup", async () => {
    invokeMock.mockRejectedValue({ kind: "commandFailed", message: XSS, raw: XSS });
    const root = await loadApp();

    expect(root.querySelector("img")).toBeNull();
    expect(document.querySelector("img")).toBeNull();
    expect(root.children).toHaveLength(0);
    expect(root.textContent).toBe(strings.errors.commandFailed);
    expect(root.textContent).not.toContain(XSS);
  });

  it("shows a raw string rejection containing markup as the generic text, with no element", async () => {
    invokeMock.mockRejectedValue(XSS);
    const root = await loadApp();

    expect(root.querySelector("img")).toBeNull();
    expect(root.textContent).toBe(strings.errors.generic);
  });
});
