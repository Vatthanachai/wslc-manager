// The single resource module for all user-visible text (English only, D13).
export const strings = {
  appTitle: "Aiyara Container Dashboard",
  containersHeading: "Containers",
  loading: "Loading...",
  empty: "No containers.",
  columns: {
    name: "Name",
    image: "Image",
    state: "State",
    status: "Status",
    session: "Session",
  },
  states: {
    running: "Running",
    exited: "Exited",
    unknown: "Unknown",
  },
  errors: {
    parseError: "The container engine returned output that could not be read.",
    commandFailed: "The container engine command failed.",
    timeout: "The container engine did not respond in time.",
    internal: "An internal error occurred.",
    generic: "Something went wrong.",
  },
} as const;
