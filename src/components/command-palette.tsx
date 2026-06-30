import { useHotkey } from "@tanstack/react-hotkeys";

import {
  Command,
  CommandDialog,
  CommandEmpty,
  CommandGroup,
  CommandInput,
  CommandItem,
  CommandList,
  CommandShortcut,
} from "@/components/ui/command";
import { useRequestStore } from "@/lib/store";

interface PaletteCommand {
  disabled?: boolean;
  id: string;
  label: string;
  run: () => void;
  shortcut?: string;
}

export function CommandPalette() {
  const open = useRequestStore((s) => s.commandsOpen);
  const setOpen = useRequestStore((s) => s.setCommandsOpen);
  const toggle = useRequestStore((s) => s.toggleCommands);
  const url = useRequestStore((s) => s.url);
  const loading = useRequestStore((s) => s.loading);
  const send = useRequestStore((s) => s.send);
  const setHistoryOpen = useRequestStore((s) => s.setHistoryOpen);
  const newRequest = useRequestStore((s) => s.newRequest);
  const newTab = useRequestStore((s) => s.newTab);

  useHotkey("Mod+Shift+P", toggle);

  const commands: PaletteCommand[] = [
    {
      id: "send",
      label: "Send request",
      shortcut: "⌘↵",
      disabled: !url || loading,
      run: send,
    },
    {
      id: "search-history",
      label: "Search history…",
      shortcut: "⌘P",
      run: () => setHistoryOpen(true),
    },
    {
      id: "new-tab",
      label: "New tab",
      shortcut: "⌘T",
      run: newTab,
    },
    {
      id: "new-request",
      label: "New request",
      run: newRequest,
    },
  ];

  return (
    <CommandDialog
      description="Run a command"
      onOpenChange={setOpen}
      open={open}
      title="Commands"
    >
      <Command>
        <CommandInput placeholder="Type a command…" />
        <CommandList>
          <CommandEmpty>No commands found.</CommandEmpty>
          <CommandGroup heading="Commands">
            {commands.map((cmd) => (
              <CommandItem
                disabled={cmd.disabled}
                key={cmd.id}
                onSelect={() => {
                  setOpen(false);
                  cmd.run();
                }}
                value={cmd.label}
              >
                <span className="flex-1">{cmd.label}</span>
                {cmd.shortcut && (
                  <CommandShortcut>{cmd.shortcut}</CommandShortcut>
                )}
              </CommandItem>
            ))}
          </CommandGroup>
        </CommandList>
      </Command>
    </CommandDialog>
  );
}
