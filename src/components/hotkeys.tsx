import { type RegisterableHotkey, useHotkey } from "@tanstack/react-hotkeys";

import { COMMANDS, type Command, keysFor } from "@/lib/keymap";
import { useRequestStore } from "@/lib/store";

export function Hotkeys() {
  const keybindings = useRequestStore((s) => s.keybindings);
  return (
    <>
      {COMMANDS.map((cmd) => {
        const keys = keysFor(cmd, keybindings);
        return keys ? (
          <CommandHotkey cmd={cmd} key={cmd.id} keys={keys} />
        ) : null;
      })}
    </>
  );
}

function CommandHotkey({ cmd, keys }: { cmd: Command; keys: string }) {
  const enabled = useRequestStore((s) => cmd.enabled?.(s) ?? true);
  useHotkey(
    keys as RegisterableHotkey,
    () => cmd.run(useRequestStore.getState()),
    {
      enabled,
    }
  );
  return null;
}
