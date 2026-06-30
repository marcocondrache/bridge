import { HeaderTable } from "@/components/header-table";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";
import { JsonViewer } from "@/components/viewers/json-viewer";
import { contentType, formatBytes, parseCookies } from "@/lib/http";
import { useRequestStore } from "@/lib/store";

function ResponseBody({
  body,
  headers,
}: {
  body: string;
  headers: [string, string][];
}) {
  if (contentType(headers).includes("json")) {
    try {
      return <JsonViewer data={JSON.parse(body)} />;
    } catch {
      // ponytail: malformed JSON falls through to raw text
    }
  }
  return <pre className="whitespace-pre-wrap font-mono text-xs">{body}</pre>;
}

export function ResponsePanel() {
  const response = useRequestStore((s) => s.response);
  const error = useRequestStore((s) => s.error);

  return (
    <div className="flex h-full flex-col rounded-md border border-border">
      {error && (
        <pre className="overflow-auto p-2 text-destructive text-xs">
          {error}
        </pre>
      )}
      {response && (
        <Tabs className="h-full min-h-0 overflow-hidden" defaultValue="body">
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
                {response.status} {response.status_text}
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
          <TabsContent className="min-h-0 overflow-auto" value="body">
            <ResponseBody body={response.body} headers={response.headers} />
          </TabsContent>
          <TabsContent className="min-h-0 overflow-auto" value="headers">
            <HeaderTable rows={response.headers} />
          </TabsContent>
          <TabsContent className="min-h-0 overflow-auto" value="cookies">
            {parseCookies(response.headers).length === 0 ? (
              <div className="p-2 text-muted-foreground text-xs">
                No cookies
              </div>
            ) : (
              <HeaderTable rows={parseCookies(response.headers)} />
            )}
          </TabsContent>
        </Tabs>
      )}
    </div>
  );
}
