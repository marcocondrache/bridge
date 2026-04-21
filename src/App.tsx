import "@/App.css";
import { useEffect, useState } from "react";
import { HISTORY, type HttpMethod } from "@/lib/constants";
import { RequestPanel } from "./components/request-panel";
import { ResponsePanel } from "./components/response-panel";
import { UrlBar } from "./components/url-bar";
import Layout from "./layout";

function App() {
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

	// Global keyboard shortcuts
	useEffect(() => {
		function handleKeyDown(e: KeyboardEvent) {
			const cmd = e.metaKey || e.ctrlKey;
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
	}, []);

	return (
		<Layout>
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
		</Layout>
	);
}

export default App;
