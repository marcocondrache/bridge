import { useHotkey } from "@tanstack/react-hotkeys";
import { invoke } from "@tauri-apps/api/core";
import { useState } from "react";

import "@/App.css";
import {
  CommandPalette,
  type PaletteCommand,
} from "@/components/command-palette";
import {
  type HistoryEntry,
  HistoryPalette,
} from "@/components/history-palette";
import type { KeyValuePair } from "@/components/key-value-editor";
import { RequestPanel } from "@/components/request-panel";
import { ResponsePanel } from "@/components/response-panel";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import {
  NativeSelect,
  NativeSelectOption,
} from "@/components/ui/native-select";
import { ResizablePanel, ResizablePanelGroup } from "@/components/ui/resizable";
import { type HttpResponse, METHODS, withParams } from "@/lib/http";

function App() {
  const [method, setMethod] = useState("GET");
  const [url, setUrl] = useState("");
  const [body, setBody] = useState("");
  const [headers, setHeaders] = useState<KeyValuePair[]>([]);
  const [params, setParams] = useState<KeyValuePair[]>([]);
  const [response, setResponse] = useState<HttpResponse | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [loading, setLoading] = useState(false);
  const [historyOpen, setHistoryOpen] = useState(false);
  const [commandsOpen, setCommandsOpen] = useState(false);

  useHotkey("Mod+P", () => setHistoryOpen((o) => !o));
  useHotkey("Mod+Shift+P", () => setCommandsOpen((o) => !o));
  useHotkey("Mod+Enter", () => send(), {
    enabled: Boolean(url) && !loading,
  });

  function loadEntry(entry: HistoryEntry) {
    setMethod(entry.method);
    setUrl(entry.url);
    setBody(entry.request_body ?? "");
    setHeaders(entry.request_headers.map(([key, value]) => ({ key, value })));
    setResponse({
      status: entry.status,
      status_text: entry.status_text,
      headers: entry.response_headers,
      body: entry.response_body,
      elapsed_ms: entry.elapsed_ms,
    });
    setError(null);
  }

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
      id: "new-request",
      label: "New request",
      run: () => {
        setUrl("");
        setBody("");
        setHeaders([]);
        setParams([]);
        setResponse(null);
        setError(null);
      },
    },
  ];

  async function send() {
    if (!url || loading) {
      return;
    }
    setLoading(true);
    setError(null);
    try {
      const res = await invoke<HttpResponse>("send_request", {
        request: {
          method,
          url: withParams(url, params),
          headers: headers
            .filter((h) => h.key.trim())
            .map((h) => [h.key, h.value]),
          body: body || null,
        },
      });
      setResponse(res);
    } catch (e) {
      setError(String(e));
      setResponse(null);
    } finally {
      setLoading(false);
    }
  }

  return (
    <div className="flex h-screen flex-col gap-3 px-4 pb-4">
      <HistoryPalette
        onOpenChange={setHistoryOpen}
        onSelect={loadEntry}
        open={historyOpen}
      />
      <CommandPalette
        commands={commands}
        onOpenChange={setCommandsOpen}
        open={commandsOpen}
      />
      {/* Draggable strip clearing the overlay traffic lights */}
      <div className="h-8 shrink-0" data-tauri-drag-region />
      <div className="flex gap-2">
        <NativeSelect
          onChange={(e) => setMethod(e.target.value)}
          value={method}
        >
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

      <ResizablePanelGroup className="min-h-0 flex-1" orientation="vertical">
        <ResizablePanel defaultSize={45} minSize={20}>
          <RequestPanel
            body={body}
            headers={headers}
            method={method}
            onBodyChange={setBody}
            onHeadersChange={setHeaders}
            onParamsChange={setParams}
            params={params}
          />
        </ResizablePanel>
        <ResizablePanel defaultSize={55} minSize={20}>
          <ResponsePanel error={error} response={response} />
        </ResizablePanel>
      </ResizablePanelGroup>
    </div>
  );
}

export default App;
