import { json } from "@codemirror/lang-json";
import { EditorView } from "@codemirror/view";
import CodeMirror from "@uiw/react-codemirror";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";
import { shadcnTheme } from "@/lib/codemirror-theme";
import { SAMPLE_RESP_BODY, SAMPLE_RESP_HEADERS } from "@/lib/constants";
import { KVTable } from "./kv-table";

function StatusBadge({ status, text }: { status: number; text: string }) {
	const color =
		status >= 500
			? "text-red-500 bg-red-500/10"
			: status >= 400
				? "text-amber-500 bg-amber-500/10"
				: status >= 300
					? "text-blue-500 bg-blue-500/10"
					: "text-green-500 bg-green-500/10";

	return (
		<span className={`rounded px-1.5 py-0.5 text-xs font-semibold ${color}`}>
			{status} {text}
		</span>
	);
}

export function ResponsePanel() {
	const headersAsRows = Object.entries(SAMPLE_RESP_HEADERS).map(
		([key, value]) => ({ key, value, on: true }),
	);

	const rawText = `HTTP/1.1 200 OK\nContent-Type: application/json; charset=utf-8\nX-RateLimit-Limit: 60\n\n${SAMPLE_RESP_BODY}`;

	return (
		<Tabs
			defaultValue="body"
			className="flex min-w-0 flex-1 flex-col overflow-hidden"
		>
			<div className="flex shrink-0 items-center border-b bg-accent pl-2.5">
				<span className="mr-2.5 text-[0.6rem] tracking-widest text-muted-foreground">
					RESPONSE
				</span>
				<TabsList variant="line">
					<TabsTrigger value="body">Body</TabsTrigger>
					<TabsTrigger value="headers">Headers</TabsTrigger>
					<TabsTrigger value="raw">Raw</TabsTrigger>
				</TabsList>
				<div className="ml-auto flex items-center gap-2 pr-2.5">
					<StatusBadge status={200} text="OK" />
					<span className="text-xs text-muted-foreground">234ms</span>
					<span className="text-xs text-muted-foreground">1.2 KB</span>
				</div>
			</div>
			<div className="flex-1 overflow-y-auto">
				<TabsContent value="body" className="h-full">
					<CodeMirror
						value={SAMPLE_RESP_BODY}
						extensions={[json(), EditorView.lineWrapping, shadcnTheme]}
						theme="none"
						editable={false}
						basicSetup={{
							lineNumbers: true,
							foldGutter: true,
							highlightActiveLine: false,
						}}
						className="h-full text-sm"
					/>
				</TabsContent>
				<TabsContent value="headers">
					<KVTable rows={headersAsRows} readOnly />
				</TabsContent>
				<TabsContent value="raw" className="h-full">
					<CodeMirror
						value={rawText}
						extensions={[EditorView.lineWrapping, shadcnTheme]}
						theme="none"
						editable={false}
						basicSetup={{
							lineNumbers: true,
							highlightActiveLine: false,
						}}
						className="h-full text-sm"
					/>
				</TabsContent>
			</div>
		</Tabs>
	);
}
