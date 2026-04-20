import { SAMPLE_RESP_BODY, SAMPLE_RESP_HEADERS } from "@/lib/constants";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";
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
    <span className={`rounded px-1.5 py-0.5 text-[10px] font-semibold ${color}`}>
      {status} {text}
    </span>
  );
}

function highlightJSON(str: string): string {
  const esc = str
    .replace(/&/g, "&amp;")
    .replace(/</g, "&lt;")
    .replace(/>/g, "&gt;");

  return esc.replace(
    /("(?:\\u[0-9a-fA-F]{4}|\\[^u]|[^\\"])*"(\s*:)?|\b(?:true|false|null)\b|-?\d+(?:\.\d*)?(?:[eE][+-]?\d+)?)/g,
    (m) => {
      if (/^"/.test(m)) {
        if (/:$/.test(m))
          return `<span class="text-primary">${m}</span>`;
        return `<span class="text-green-400 dark:text-green-300">${m}</span>`;
      }
      if (/true|false/.test(m))
        return `<span class="text-orange-400 dark:text-orange-300">${m}</span>`;
      if (/null/.test(m))
        return `<span class="text-muted-foreground">${m}</span>`;
      return `<span class="text-pink-400 dark:text-pink-300">${m}</span>`;
    },
  );
}

export function ResponsePanel() {
  const headersAsRows = Object.entries(SAMPLE_RESP_HEADERS).map(
    ([key, value]) => ({ key, value, on: true }),
  );

  const rawText = `HTTP/1.1 200 OK\nContent-Type: application/json; charset=utf-8\nX-RateLimit-Limit: 60\n\n${SAMPLE_RESP_BODY}`;

  return (
    <Tabs defaultValue="body" className="flex min-w-0 flex-1 flex-col overflow-hidden">
      <div className="flex shrink-0 items-center border-b bg-accent pl-2.5">
        <span className="mr-2.5 text-[9px] tracking-[0.12em] text-muted-foreground">
          RESPONSE
        </span>
        <TabsList variant="line">
          <TabsTrigger value="body">Body</TabsTrigger>
          <TabsTrigger value="headers">Headers</TabsTrigger>
          <TabsTrigger value="raw">Raw</TabsTrigger>
        </TabsList>
        <div className="ml-auto flex items-center gap-2 pr-2.5">
          <StatusBadge status={200} text="OK" />
          <span className="text-[9px] text-muted-foreground">234ms</span>
          <span className="text-[9px] text-muted-foreground">1.2 KB</span>
        </div>
      </div>
      <div className="flex-1 overflow-y-auto">
        <TabsContent value="body">
          <pre
            className="overflow-x-auto p-3 text-[11px] leading-[1.75] text-muted-foreground"
            dangerouslySetInnerHTML={{ __html: highlightJSON(SAMPLE_RESP_BODY) }}
          />
        </TabsContent>
        <TabsContent value="headers">
          <KVTable rows={headersAsRows} readOnly />
        </TabsContent>
        <TabsContent value="raw">
          <pre className="break-all p-3 text-[11px] leading-[1.75] whitespace-pre-wrap text-muted-foreground">
            {rawText}
          </pre>
        </TabsContent>
      </div>
    </Tabs>
  );
}
