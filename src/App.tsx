import { invoke } from "@tauri-apps/api/core";
import { useState } from "react";

import "@/App.css";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import {
	NativeSelect,
	NativeSelectOption,
} from "@/components/ui/native-select";
import { Textarea } from "@/components/ui/textarea";

const METHODS = ["GET", "POST", "PUT", "PATCH", "DELETE", "HEAD", "OPTIONS"];

type HttpResponse = {
	status: number;
	headers: [string, string][];
	body: string;
	elapsed_ms: number;
};

function App() {
	const [method, setMethod] = useState("GET");
	const [url, setUrl] = useState("");
	const [body, setBody] = useState("");
	const [response, setResponse] = useState<HttpResponse | null>(null);
	const [error, setError] = useState<string | null>(null);
	const [loading, setLoading] = useState(false);

	async function send() {
		if (!url || loading) return;
		setLoading(true);
		setError(null);
		try {
			const res = await invoke<HttpResponse>("send_request", {
				request: {
					method,
					url,
					headers: [],
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
		<div className="flex h-screen flex-col gap-3 p-4">
			<div className="flex gap-2">
				<NativeSelect
					value={method}
					onChange={(e) => setMethod(e.target.value)}
				>
					{METHODS.map((m) => (
						<NativeSelectOption key={m} value={m}>
							{m}
						</NativeSelectOption>
					))}
				</NativeSelect>
				<Input
					value={url}
					onChange={(e) => setUrl(e.target.value)}
					onKeyDown={(e) => {
						if (e.key === "Enter" && (e.metaKey || e.ctrlKey)) send();
					}}
					placeholder="https://api.example.com"
					className="flex-1"
				/>
				<Button onClick={send} disabled={loading}>
					{loading ? "Sending…" : "Send"}
				</Button>
			</div>

			{method !== "GET" && method !== "HEAD" && (
				<Textarea
					value={body}
					onChange={(e) => setBody(e.target.value)}
					placeholder="Request body"
					className="h-24 font-mono"
				/>
			)}

			<div className="flex-1 overflow-auto rounded-md border border-border p-2">
				{error && <pre className="text-xs text-destructive">{error}</pre>}
				{response && (
					<div className="flex flex-col gap-2">
						<div className="text-xs text-muted-foreground">
							{response.status} · {response.elapsed_ms}ms
						</div>
						<pre className="font-mono text-xs whitespace-pre-wrap">
							{response.body}
						</pre>
					</div>
				)}
			</div>
		</div>
	);
}

export default App;
