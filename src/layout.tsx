import { useState } from "react";
import { SidebarProvider } from "@/components/ui/sidebar";
import { HistorySidebar } from "./components/history-sidebar";
import { StatusBar } from "./components/status-bar";
import { TitleBar } from "./components/title-bar";
import { HISTORY } from "./lib/constants";

export default function Layout({ children }: { children: React.ReactNode }) {
	const [selectedIndex, setSelectedIndex] = useState(0);

	function handleSelect(index: number) {
		setSelectedIndex(index);
	}

	return (
		<div className="flex h-screen flex-col overflow-hidden bg-background">
			<TitleBar />
			<SidebarProvider className="relative flex-1 min-h-0">
				<HistorySidebar
					items={HISTORY}
					selectedIndex={selectedIndex}
					onSelect={handleSelect}
				/>
				{children}
			</SidebarProvider>
			<StatusBar />
		</div>
	);
}
