import type { RequestState } from "@/lib/store";

export interface Command {
  defaultKeys: string;
  enabled?: (s: RequestState) => boolean;
  hidden?: boolean;
  id: string;
  name: string;
  run: (s: RequestState) => void;
}

export const COMMANDS: Command[] = [
  {
    id: "send",
    name: "Send request",
    defaultKeys: "Mod+Enter",
    run: (s) => s.send(),
    enabled: (s) => Boolean(s.url) && !s.loading,
  },
  {
    id: "search-history",
    name: "Search history",
    defaultKeys: "Mod+P",
    run: (s) => s.toggleHistory(),
  },
  {
    id: "commands",
    name: "Command palette",
    defaultKeys: "Mod+Shift+P",
    run: (s) => s.toggleCommands(),
    hidden: true,
  },
  {
    id: "new-tab",
    name: "New tab",
    defaultKeys: "Mod+T",
    run: (s) => s.newTab(),
  },
  {
    id: "close-tab",
    name: "Close tab",
    defaultKeys: "Mod+W",
    run: (s) => s.closeTab(s.activeTabId),
  },
  {
    id: "new-request",
    name: "New request",
    defaultKeys: "",
    run: (s) => s.newRequest(),
  },
];

export const keysFor = (
  cmd: Command,
  keybindings: Record<string, string>
): string => keybindings[cmd.id] ?? cmd.defaultKeys;
