import { useHotkey } from "@tanstack/react-hotkeys";
import { invoke } from "@tauri-apps/api/core";
import { useEffect, useState } from "react";

import {
  Command,
  CommandDialog,
  CommandEmpty,
  CommandGroup,
  CommandInput,
  CommandItem,
  CommandList,
} from "@/components/ui/command";
import { useRequestStore } from "@/lib/store";

export interface HistoryEntry {
  created_at: number;
  elapsed_ms: number;
  id: number;
  method: string;
  request_body: string | null;
  request_headers: [string, string][];
  response_body: string;
  response_headers: [string, string][];
  status: number;
  status_text: string;
  url: string;
}

function timeAgo(ms: number): string {
  const s = Math.max(0, (Date.now() - ms) / 1000);
  if (s < 60) {
    return "just now";
  }
  const m = s / 60;
  if (m < 60) {
    return `${Math.floor(m)}m ago`;
  }
  const h = m / 60;
  if (h < 24) {
    return `${Math.floor(h)}h ago`;
  }
  return `${Math.floor(h / 24)}d ago`;
}

export function HistoryPalette() {
  const open = useRequestStore((s) => s.historyOpen);
  const onOpenChange = useRequestStore((s) => s.setHistoryOpen);
  const toggle = useRequestStore((s) => s.toggleHistory);
  const onSelect = useRequestStore((s) => s.loadEntry);
  const [search, setSearch] = useState("");
  const [entries, setEntries] = useState<HistoryEntry[]>([]);

  useHotkey("Mod+P", toggle);

  // Debounced FTS query to the Rust backend (bm25-ranked, top results).
  useEffect(() => {
    if (!open) {
      return;
    }
    const handle = setTimeout(() => {
      invoke<HistoryEntry[]>("query_history", {
        query: { search: search || null, limit: 50 },
      })
        .then(setEntries)
        .catch(() => setEntries([]));
    }, 120);
    return () => clearTimeout(handle);
  }, [search, open]);

  return (
    <CommandDialog
      description="Search your request history"
      onOpenChange={onOpenChange}
      open={open}
      title="Search history"
    >
      <Command shouldFilter={false}>
        <CommandInput
          onValueChange={setSearch}
          placeholder="Search history…"
          value={search}
        />
        <CommandList>
          <CommandEmpty>No matching requests.</CommandEmpty>
          <CommandGroup heading={search ? "Results" : "Recent"}>
            {entries.map((entry) => (
              <CommandItem
                key={entry.id}
                onSelect={() => {
                  onSelect(entry);
                  onOpenChange(false);
                }}
                value={String(entry.id)}
              >
                <span className="w-12 shrink-0 font-mono text-muted-foreground text-xs">
                  {entry.method}
                </span>
                <span className="flex-1 truncate">{entry.url}</span>
                <span className="shrink-0 font-mono text-muted-foreground text-xs">
                  {entry.status}
                </span>
                <span className="w-16 shrink-0 text-right text-muted-foreground text-xs">
                  {timeAgo(entry.created_at)}
                </span>
              </CommandItem>
            ))}
          </CommandGroup>
        </CommandList>
      </Command>
    </CommandDialog>
  );
}
