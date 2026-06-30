import { KeyValueEditor } from "@/components/key-value-editor";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";
import { Textarea } from "@/components/ui/textarea";
import { useRequestStore } from "@/lib/store";

export function RequestPanel() {
  const method = useRequestStore((s) => s.method);
  const headers = useRequestStore((s) => s.headers);
  const params = useRequestStore((s) => s.params);
  const body = useRequestStore((s) => s.body);
  const setHeaders = useRequestStore((s) => s.setHeaders);
  const setParams = useRequestStore((s) => s.setParams);
  const setBody = useRequestStore((s) => s.setBody);

  const hasBody = method !== "GET" && method !== "HEAD";

  return (
    <Tabs className="h-full min-h-0 overflow-hidden" defaultValue="headers">
      <TabsList>
        <TabsTrigger value="headers">Headers</TabsTrigger>
        <TabsTrigger value="params">Params</TabsTrigger>
        {hasBody && <TabsTrigger value="body">Body</TabsTrigger>}
      </TabsList>
      <TabsContent className="min-h-0 overflow-auto" value="headers">
        <KeyValueEditor noun="header" onChange={setHeaders} pairs={headers} />
      </TabsContent>
      <TabsContent className="min-h-0 overflow-auto" value="params">
        <KeyValueEditor noun="param" onChange={setParams} pairs={params} />
      </TabsContent>
      {hasBody && (
        <TabsContent
          className="flex min-h-0 flex-col overflow-auto"
          value="body"
        >
          <Textarea
            className="flex-1 font-mono"
            onChange={(e) => setBody(e.target.value)}
            placeholder="Request body"
            value={body}
          />
        </TabsContent>
      )}
    </Tabs>
  );
}
