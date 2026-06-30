import "@/App.css";
import { CommandPalette } from "@/components/command-palette";
import { HistoryPalette } from "@/components/history-palette";
import { Hotkeys } from "@/components/hotkeys";
import { RequestBar } from "@/components/request-bar";
import { RequestPanel } from "@/components/request-panel";
import { ResponsePanel } from "@/components/response-panel";
import { TabBar } from "@/components/tab-bar";
import { ResizablePanel, ResizablePanelGroup } from "@/components/ui/resizable";

function App() {
  return (
    <div className="flex h-screen flex-col gap-3 px-4 pb-4">
      <Hotkeys />
      <HistoryPalette />
      <CommandPalette />
      {/* Draggable strip clearing the overlay traffic lights */}
      <div className="h-8 shrink-0" data-tauri-drag-region />
      <TabBar />
      <RequestBar />
      <ResizablePanelGroup className="min-h-0 flex-1" orientation="vertical">
        <ResizablePanel defaultSize={45} minSize={20}>
          <RequestPanel />
        </ResizablePanel>
        <ResizablePanel defaultSize={55} minSize={20}>
          <ResponsePanel />
        </ResizablePanel>
      </ResizablePanelGroup>
    </div>
  );
}

export default App;
