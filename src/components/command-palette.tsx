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

export interface PaletteCommand {
  disabled?: boolean;
  id: string;
  label: string;
  run: () => void;
  shortcut?: string;
}

interface Props {
  commands: PaletteCommand[];
  onOpenChange: (open: boolean) => void;
  open: boolean;
}

export function CommandPalette({ open, onOpenChange, commands }: Props) {
  return (
    <CommandDialog
      description="Run a command"
      onOpenChange={onOpenChange}
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
                  onOpenChange(false);
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
