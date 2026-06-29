import { useHotkey } from "@tanstack/react-hotkeys";
import { invoke } from "@tauri-apps/api/core";
import { useState } from "react";

import "@/App.css";
import {
	CommandPalette,
	type PaletteCommand,
} from "@/components/command-palette";
import {
	type KeyValuePair,
	KeyValueEditor,
} from "@/components/key-value-editor";
import {
	type HistoryEntry,
	HistoryPalette,
} from "@/components/history-palette";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import {
	NativeSelect,
	NativeSelectOption,
} from "@/components/ui/native-select";
import { ResizablePanel, ResizablePanelGroup } from "@/components/ui/resizable";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";
import { Textarea } from "@/components/ui/textarea";

const METHODS = ["GET", "POST", "PUT", "PATCH", "DELETE", "HEAD", "OPTIONS"];

function withParams(url: string, params: KeyValuePair[]): string {
	const qs = new URLSearchParams(
		params.filter((p) => p.key.trim()).map((p) => [p.key, p.value]),
	).toString();
	if (!qs) return url;
	return url + (url.includes("?") ? "&" : "?") + qs;
}

type HttpResponse = {
	status: number;
	headers: [string, string][];
	body: string;
	elapsed_ms: number;
};

const STATUS_TEXT: Record<number, string> = {
	200: "OK",
	201: "Created",
	204: "No Content",
	301: "Moved Permanently",
	302: "Found",
	304: "Not Modified",
	400: "Bad Request",
	401: "Unauthorized",
	403: "Forbidden",
	404: "Not Found",
	405: "Method Not Allowed",
	409: "Conflict",
	422: "Unprocessable Entity",
	429: "Too Many Requests",
	500: "Internal Server Error",
	502: "Bad Gateway",
	503: "Service Unavailable",
	504: "Gateway Timeout",
};

function statusText(status: number): string {
	return STATUS_TEXT[status] ?? "";
}

function formatBytes(bytes: number): string {
	if (bytes < 1024) return `${bytes} B`;
	if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`;
	return `${(bytes / (1024 * 1024)).toFixed(1)} MB`;
}

function contentType(headers: [string, string][]): string {
	const ct = headers.find(([k]) => k.toLowerCase() === "content-type")?.[1];
	return ct?.split(";")[0]?.trim() ?? "";
}

function parseCookies(headers: [string, string][]): [string, string][] {
	return headers
		.filter(([k]) => k.toLowerCase() === "set-cookie")
		.map(([, v]) => {
			const [pair, ...attrs] = v.split(";");
			const eq = pair.indexOf("=");
			const name = eq === -1 ? pair.trim() : pair.slice(0, eq).trim();
			const value = eq === -1 ? "" : pair.slice(eq + 1).trim();
			const meta = attrs.map((a) => a.trim()).join("; ");
			return [name, meta ? `${value}  ·  ${meta}` : value];
		});
}

function HeaderTable({ rows }: { rows: [string, string][] }) {
	return (
		<table className="w-full text-xs">
			<tbody>
				{rows.map(([key, value], i) => (
					<tr key={`${key}-${i}`} className="align-top">
						<td className="py-1 pr-3 font-medium whitespace-nowrap text-muted-foreground">
							{key}
						</td>
						<td className="py-1 font-mono break-all">{value}</td>
					</tr>
				))}
			</tbody>
		</table>
	);
}

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
		if (!url || loading) return;
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
				open={historyOpen}
				onOpenChange={setHistoryOpen}
				onSelect={loadEntry}
			/>
			<CommandPalette
				open={commandsOpen}
				onOpenChange={setCommandsOpen}
				commands={commands}
			/>
			{/* Draggable strip clearing the overlay traffic lights */}
			<div data-tauri-drag-region className="h-8 shrink-0" />
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
					placeholder="https://api.example.com"
					className="flex-1"
				/>
				<Button onClick={send} disabled={loading}>
					{loading ? "Sending…" : "Send"}
				</Button>
			</div>

			<ResizablePanelGroup orientation="vertical" className="min-h-0 flex-1">
				<ResizablePanel defaultSize={45} minSize={20}>
					<Tabs
						defaultValue="headers"
						className="h-full min-h-0 overflow-hidden"
					>
						<TabsList>
							<TabsTrigger value="headers">Headers</TabsTrigger>
							<TabsTrigger value="params">Params</TabsTrigger>
							{method !== "GET" && method !== "HEAD" && (
								<TabsTrigger value="body">Body</TabsTrigger>
							)}
						</TabsList>
						<TabsContent value="headers" className="min-h-0 overflow-auto">
							<KeyValueEditor
								pairs={headers}
								onChange={setHeaders}
								noun="header"
							/>
						</TabsContent>
						<TabsContent value="params" className="min-h-0 overflow-auto">
							<KeyValueEditor
								pairs={params}
								onChange={setParams}
								noun="param"
							/>
						</TabsContent>
						{method !== "GET" && method !== "HEAD" && (
							<TabsContent
								value="body"
								className="flex min-h-0 flex-col overflow-auto"
							>
								<Textarea
									value={body}
									onChange={(e) => setBody(e.target.value)}
									placeholder="Request body"
									className="flex-1 font-mono"
								/>
							</TabsContent>
						)}
					</Tabs>
				</ResizablePanel>
				<ResizablePanel defaultSize={55} minSize={20}>
					<div className="flex h-full flex-col rounded-md border border-border">
						{error && (
							<pre className="overflow-auto p-2 text-xs text-destructive">
								{error}
							</pre>
						)}
						{response && (
							<Tabs
								defaultValue="body"
								className="h-full min-h-0 overflow-hidden"
							>
								<div className="flex items-center justify-between gap-2">
									<TabsList>
										<TabsTrigger value="body">Body</TabsTrigger>
										<TabsTrigger value="headers">
											Headers ({response.headers.length})
										</TabsTrigger>
										<TabsTrigger value="cookies">
											Cookies ({parseCookies(response.headers).length})
										</TabsTrigger>
									</TabsList>
									<div className="flex shrink-0 items-center gap-3 pr-1 text-xs">
										<span
											className={
												response.status < 400
													? "font-medium text-emerald-600 dark:text-emerald-400"
													: "font-medium text-destructive"
											}
										>
											{response.status} {statusText(response.status)}
										</span>
										<span className="text-muted-foreground">
											{response.elapsed_ms}ms
										</span>
										<span className="text-muted-foreground">
											{formatBytes(new Blob([response.body]).size)}
										</span>
										{contentType(response.headers) && (
											<span className="truncate text-muted-foreground">
												{contentType(response.headers)}
											</span>
										)}
									</div>
								</div>
								<TabsContent value="body" className="min-h-0 overflow-auto">
									<pre className="font-mono text-xs whitespace-pre-wrap">
										{response.body}
									</pre>
								</TabsContent>
								<TabsContent
									value="headers"
									className="min-h-0 overflow-auto"
								>
									<HeaderTable rows={response.headers} />
								</TabsContent>
								<TabsContent
									value="cookies"
									className="min-h-0 overflow-auto"
								>
									{parseCookies(response.headers).length === 0 ? (
										<div className="p-2 text-xs text-muted-foreground">
											No cookies
										</div>
									) : (
										<HeaderTable rows={parseCookies(response.headers)} />
									)}
								</TabsContent>
							</Tabs>
						)}
					</div>
				</ResizablePanel>
			</ResizablePanelGroup>
		</div>
	);
}

export default App;
