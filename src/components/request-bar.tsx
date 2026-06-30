import { useHotkey } from "@tanstack/react-hotkeys";

import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import {
  NativeSelect,
  NativeSelectOption,
} from "@/components/ui/native-select";
import { METHODS } from "@/lib/http";
import { useRequestStore } from "@/lib/store";

export function RequestBar() {
  const method = useRequestStore((s) => s.method);
  const url = useRequestStore((s) => s.url);
  const loading = useRequestStore((s) => s.loading);
  const setMethod = useRequestStore((s) => s.setMethod);
  const setUrl = useRequestStore((s) => s.setUrl);
  const send = useRequestStore((s) => s.send);

  useHotkey("Mod+Enter", () => send(), {
    enabled: Boolean(url) && !loading,
  });

  return (
    <div className="flex gap-2">
      <NativeSelect onChange={(e) => setMethod(e.target.value)} value={method}>
        {METHODS.map((m) => (
          <NativeSelectOption key={m} value={m}>
            {m}
          </NativeSelectOption>
        ))}
      </NativeSelect>
      <Input
        className="flex-1"
        onChange={(e) => setUrl(e.target.value)}
        placeholder="https://api.example.com"
        value={url}
      />
      <Button disabled={loading} onClick={send}>
        {loading ? "Sending…" : "Send"}
      </Button>
    </div>
  );
}
