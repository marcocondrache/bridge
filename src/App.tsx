import "@/App.css";
import { useEffect, useState } from "react";
import { HISTORY, type HttpMethod } from "@/lib/constants";
import { useThemeStore } from "@/state/theme";
import { HistorySidebar } from "./components/history-sidebar";
import { RequestPanel } from "./components/request-panel";
import { ResponsePanel } from "./components/response-panel";
import { StatusBar } from "./components/status-bar";
import { TitleBar } from "./components/title-bar";
import { UrlBar } from "./components/url-bar";

function App() {
	const { dark, toggleTheme } = useThemeStore();
	const [selectedIndex, setSelectedIndex] = useState(0);
	const [url, setUrl] = useState(HISTORY[0].url);
	const [method, setMethod] = useState<HttpMethod>(HISTORY[0].method);

	function navigate(index: number) {
		setUrl(HISTORY[index].url);
		setMethod(HISTORY[index].method);
	}

	function handleSelect(index: number) {
		setSelectedIndex(index);
		navigate(index);
	}

	function handleSend() {
		// TODO: implement actual request sending
	}

	// Apply dark class to document
	useEffect(() => {
		document.documentElement.classList.toggle("dark", dark);
	}, [dark]);

	// Global keyboard shortcuts
	useEffect(() => {
		function handleKeyDown(e: KeyboardEvent) {
			const cmd = e.metaKey || e.ctrlKey;
			if (cmd && e.key === "d") {
				e.preventDefault();
				toggleTheme();
			}
			if (cmd && e.key === "ArrowUp") {
				e.preventDefault();
				setSelectedIndex((i) => {
					const n = Math.max(i - 1, 0);
					navigate(n);
					return n;
				});
			}
			if (cmd && e.key === "ArrowDown") {
				e.preventDefault();
				setSelectedIndex((i) => {
					const n = Math.min(i + 1, HISTORY.length - 1);
					navigate(n);
					return n;
				});
			}
		}
		window.addEventListener("keydown", handleKeyDown);
		return () => window.removeEventListener("keydown", handleKeyDown);
	}, [toggleTheme]);

	return (
		<main className="flex h-screen flex-col overflow-hidden bg-background">
			<TitleBar dark={dark} onToggleTheme={toggleTheme} />

			<div className="flex flex-1 overflow-hidden">
				<HistorySidebar
					items={HISTORY}
					selectedIndex={selectedIndex}
					onSelect={handleSelect}
				/>

				<div className="flex flex-1 flex-col overflow-hidden">
					<UrlBar
						url={url}
						method={method}
						onUrlChange={setUrl}
						onMethodChange={setMethod}
						onSend={handleSend}
					/>

					<div className="flex flex-1 overflow-hidden border-t">
						<RequestPanel />
						<div className="w-px shrink-0 bg-border" />
						<ResponsePanel />
					</div>
				</div>
			</div>

			<StatusBar />
		</main>
	);
}

export default App;
