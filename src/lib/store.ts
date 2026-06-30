import { invoke } from "@tauri-apps/api/core";
import { create } from "zustand";

import type { HistoryEntry } from "@/components/history-palette";
import type { KeyValuePair } from "@/components/key-value-editor";
import { type HttpResponse, withParams } from "@/lib/http";

interface RequestState {
  body: string;
  commandsOpen: boolean;
  error: string | null;
  headers: KeyValuePair[];
  historyOpen: boolean;
  loadEntry: (entry: HistoryEntry) => void;
  loading: boolean;
  method: string;
  newRequest: () => void;
  params: KeyValuePair[];
  response: HttpResponse | null;

  send: () => Promise<void>;
  setBody: (body: string) => void;
  setCommandsOpen: (open: boolean) => void;
  setHeaders: (headers: KeyValuePair[]) => void;
  setHistoryOpen: (open: boolean) => void;

  setMethod: (method: string) => void;
  setParams: (params: KeyValuePair[]) => void;
  setUrl: (url: string) => void;
  toggleCommands: () => void;
  toggleHistory: () => void;
  url: string;
}

export const useRequestStore = create<RequestState>((set, get) => ({
  method: "GET",
  url: "",
  body: "",
  headers: [],
  params: [],
  response: null,
  error: null,
  loading: false,
  historyOpen: false,
  commandsOpen: false,

  setMethod: (method) => set({ method }),
  setUrl: (url) => set({ url }),
  setBody: (body) => set({ body }),
  setHeaders: (headers) => set({ headers }),
  setParams: (params) => set({ params }),
  setHistoryOpen: (historyOpen) => set({ historyOpen }),
  toggleHistory: () => set((s) => ({ historyOpen: !s.historyOpen })),
  setCommandsOpen: (commandsOpen) => set({ commandsOpen }),
  toggleCommands: () => set((s) => ({ commandsOpen: !s.commandsOpen })),

  send: async () => {
    const { url, loading, method, params, headers, body } = get();
    if (!url || loading) {
      return;
    }
    set({ loading: true, error: null });
    try {
      const response = await invoke<HttpResponse>("send_request", {
        request: {
          method,
          url: withParams(url, params),
          headers: headers
            .filter((h) => h.key.trim())
            .map((h) => [h.key, h.value]),
          body: body || null,
        },
      });
      set({ response });
    } catch (e) {
      set({ error: String(e), response: null });
    } finally {
      set({ loading: false });
    }
  },

  loadEntry: (entry) =>
    set({
      method: entry.method,
      url: entry.url,
      body: entry.request_body ?? "",
      headers: entry.request_headers.map(([key, value]) => ({ key, value })),
      response: {
        status: entry.status,
        status_text: entry.status_text,
        headers: entry.response_headers,
        body: entry.response_body,
        elapsed_ms: entry.elapsed_ms,
      },
      error: null,
    }),

  newRequest: () =>
    set({
      url: "",
      body: "",
      headers: [],
      params: [],
      response: null,
      error: null,
    }),
}));
