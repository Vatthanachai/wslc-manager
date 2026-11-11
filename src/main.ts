import { invoke } from "@tauri-apps/api/core";
import { strings } from "./strings";

type ContainerState =
  | { kind: "running" }
  | { kind: "exited" }
  | { kind: "unknown"; raw: string };

interface Container {
  id: string;
  name: string;
  image: string;
  state: ContainerState;
  status_text: string | null;
  scope: { id: string; display_name: string };
}

interface AppError {
  kind: "parseError" | "commandFailed" | "timeout" | "internal";
}

function stateText(s: ContainerState): string {
  if (s.kind === "running") return strings.states.running;
  if (s.kind === "exited") return strings.states.exited;
  return `${strings.states.unknown} (${s.raw})`;
}

function errorText(e: unknown): string {
  const kind = (e as AppError | undefined)?.kind;
  switch (kind) {
    case "parseError":
      return strings.errors.parseError;
    case "commandFailed":
      return strings.errors.commandFailed;
    case "timeout":
      return strings.errors.timeout;
    case "internal":
      return strings.errors.internal;
    default:
      return strings.errors.generic;
  }
}

function cell(tag: "td" | "th", text: string): HTMLElement {
  const el = document.createElement(tag);
  el.textContent = text;
  return el;
}

function render(root: HTMLElement, containers: Container[]): void {
  root.replaceChildren();
  const h1 = document.createElement("h1");
  h1.textContent = strings.containersHeading;
  root.append(h1);
  if (containers.length === 0) {
    const p = document.createElement("p");
    p.textContent = strings.empty;
    root.append(p);
    return;
  }
  const table = document.createElement("table");
  const head = table.createTHead().insertRow();
  const c = strings.columns;
  [c.name, c.image, c.state, c.status, c.session].forEach((t) => head.append(cell("th", t)));
  const body = table.createTBody();
  for (const ct of containers) {
    const row = body.insertRow();
    row.append(
      cell("td", ct.name),
      cell("td", ct.image),
      cell("td", stateText(ct.state)),
      cell("td", ct.status_text ?? ""),
      cell("td", ct.scope.display_name),
    );
  }
  root.append(table);
}

async function main(): Promise<void> {
  document.title = strings.appTitle;
  const root = document.getElementById("app")!;
  root.textContent = strings.loading;
  try {
    render(root, await invoke<Container[]>("list_containers"));
  } catch (e) {
    root.textContent = errorText(e);
  }
}

void main();
