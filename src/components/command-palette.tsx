import { formatForDisplay } from "@tanstack/react-hotkeys";

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
import { COMMANDS, keysFor } from "@/lib/keymap";
import { useRequestStore } from "@/lib/store";

export function CommandPalette() {
  const setOpen = useRequestStore((s) => s.setCommandsOpen);
  const state = useRequestStore();

  const commands = COMMANDS.filter((c) => !c.hidden);

  return (
    <CommandDialog
      description="Run a command"
      onOpenChange={setOpen}
      open={state.commandsOpen}
      title="Commands"
    >
      <Command>
        <CommandInput placeholder="Type a command…" />
        <CommandList>
          <CommandEmpty>No commands found.</CommandEmpty>
          <CommandGroup heading="Commands">
            {commands.map((cmd) => {
              const keys = keysFor(cmd, state.keybindings);
              const disabled = !(cmd.enabled?.(state) ?? true);
              return (
                <CommandItem
                  disabled={disabled}
                  key={cmd.id}
                  onSelect={() => {
                    setOpen(false);
                    cmd.run(state);
                  }}
                  value={cmd.name}
                >
                  <span className="flex-1">{cmd.name}</span>
                  {keys && (
                    <CommandShortcut>{formatForDisplay(keys)}</CommandShortcut>
                  )}
                </CommandItem>
              );
            })}
          </CommandGroup>
        </CommandList>
      </Command>
    </CommandDialog>
  );
}
