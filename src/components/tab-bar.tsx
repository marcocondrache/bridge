import { useHotkey } from "@tanstack/react-hotkeys";

import { Tabs, TabsList, TabsTrigger } from "@/components/ui/tabs";
import { useRequestStore } from "@/lib/store";

export function TabBar() {
  const tabs = useRequestStore((s) => s.tabs);
  const activeTabId = useRequestStore((s) => s.activeTabId);
  const liveUrl = useRequestStore((s) => s.url);
  const liveMethod = useRequestStore((s) => s.method);
  const newTab = useRequestStore((s) => s.newTab);
  const closeTab = useRequestStore((s) => s.closeTab);
  const switchTab = useRequestStore((s) => s.switchTab);

  useHotkey("Mod+T", newTab);
  useHotkey("Mod+W", () => closeTab(activeTabId));

  return (
    <Tabs
      onValueChange={(v) => switchTab(Number(v))}
      value={String(activeTabId)}
    >
      <TabsList className="w-full justify-start overflow-x-auto" variant="line">
        {tabs.map((tab) => {
          const active = tab.id === activeTabId;
          const method = active ? liveMethod : tab.state.method;
          const url = active ? liveUrl : tab.state.url;
          return (
            <div className="relative flex-none" key={tab.id}>
              <TabsTrigger className="gap-1.5 pr-6" value={String(tab.id)}>
                <span className="font-mono text-[10px] opacity-60">
                  {method}
                </span>
                <span className="max-w-40 truncate">
                  {url || "New request"}
                </span>
              </TabsTrigger>
              {tabs.length > 1 && (
                <button
                  aria-label="Close tab"
                  className="absolute top-1/2 right-1 -translate-y-1/2 rounded-sm px-0.5 text-foreground/40 hover:text-foreground"
                  onClick={() => closeTab(tab.id)}
                  type="button"
                >
                  ×
                </button>
              )}
            </div>
          );
        })}
        <button
          aria-label="New tab"
          className="ml-1 px-1.5 text-foreground/60 text-sm hover:text-foreground"
          onClick={newTab}
          type="button"
        >
          +
        </button>
      </TabsList>
    </Tabs>
  );
}
