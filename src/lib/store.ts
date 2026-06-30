import { invoke } from "@tauri-apps/api/core";
import { create } from "zustand";

import type { HistoryEntry } from "@/components/history-palette";
import type { KeyValuePair } from "@/components/key-value-editor";
import { type HttpResponse, withParams } from "@/lib/http";

// Fields that belong to a single request tab. Top-level store fields hold the
// live active tab; `tabs` keeps snapshots, refreshed when switching away.
interface TabState {
  body: string;
  error: string | null;
  headers: KeyValuePair[];
  loading: boolean;
  method: string;
  params: KeyValuePair[];
  response: HttpResponse | null;
  url: string;
}

interface Tab {
  id: number;
  state: TabState;
}

const blankTab = (): TabState => ({
  method: "GET",
  url: "",
  body: "",
  headers: [],
  params: [],
  response: null,
  error: null,
  loading: false,
});

const snapshot = (s: TabState): TabState => ({
  method: s.method,
  url: s.url,
  body: s.body,
  headers: s.headers,
  params: s.params,
  response: s.response,
  error: s.error,
  loading: s.loading,
});

interface RequestState extends TabState {
  activeTabId: number;
  closeTab: (id: number) => void;
  commandsOpen: boolean;
  historyOpen: boolean;
  loadEntry: (entry: HistoryEntry) => void;
  newRequest: () => void;
  newTab: () => void;
  send: () => Promise<void>;
  setBody: (body: string) => void;
  setCommandsOpen: (open: boolean) => void;
  setHeaders: (headers: KeyValuePair[]) => void;
  setHistoryOpen: (open: boolean) => void;
  setMethod: (method: string) => void;
  setParams: (params: KeyValuePair[]) => void;
  setUrl: (url: string) => void;
  switchTab: (id: number) => void;
  tabs: Tab[];
  toggleCommands: () => void;
  toggleHistory: () => void;
}

export const useRequestStore = create<RequestState>((set, get) => ({
  ...blankTab(),
  tabs: [{ id: 0, state: blankTab() }],
  activeTabId: 0,
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

  newTab: () =>
    set((s) => {
      const id = Math.max(...s.tabs.map((t) => t.id)) + 1;
      return {
        tabs: [
          ...s.tabs.map((t) =>
            t.id === s.activeTabId ? { ...t, state: snapshot(s) } : t
          ),
          { id, state: blankTab() },
        ],
        activeTabId: id,
        ...blankTab(),
      };
    }),

  switchTab: (id) =>
    set((s) => {
      if (id === s.activeTabId) {
        return {};
      }
      const target = s.tabs.find((t) => t.id === id);
      if (!target) {
        return {};
      }
      return {
        tabs: s.tabs.map((t) =>
          t.id === s.activeTabId ? { ...t, state: snapshot(s) } : t
        ),
        activeTabId: id,
        ...target.state,
      };
    }),

  closeTab: (id) =>
    set((s) => {
      // ponytail: always keep at least one tab open
      if (s.tabs.length === 1) {
        return {};
      }
      const idx = s.tabs.findIndex((t) => t.id === id);
      const remaining = s.tabs.filter((t) => t.id !== id);
      if (id !== s.activeTabId) {
        return { tabs: remaining };
      }
      const next = remaining[Math.min(idx, remaining.length - 1)];
      return { tabs: remaining, activeTabId: next.id, ...next.state };
    }),
}));
