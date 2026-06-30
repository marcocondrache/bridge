import {
  KeyValueEditor,
  type KeyValuePair,
} from "@/components/key-value-editor";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";
import { Textarea } from "@/components/ui/textarea";

interface Props {
  body: string;
  headers: KeyValuePair[];
  method: string;
  onBodyChange: (body: string) => void;
  onHeadersChange: (pairs: KeyValuePair[]) => void;
  onParamsChange: (pairs: KeyValuePair[]) => void;
  params: KeyValuePair[];
}

export function RequestPanel({
  method,
  headers,
  params,
  body,
  onHeadersChange,
  onParamsChange,
  onBodyChange,
}: Props) {
  const hasBody = method !== "GET" && method !== "HEAD";

  return (
    <Tabs className="h-full min-h-0 overflow-hidden" defaultValue="headers">
      <TabsList>
        <TabsTrigger value="headers">Headers</TabsTrigger>
        <TabsTrigger value="params">Params</TabsTrigger>
        {hasBody && <TabsTrigger value="body">Body</TabsTrigger>}
      </TabsList>
      <TabsContent className="min-h-0 overflow-auto" value="headers">
        <KeyValueEditor
          noun="header"
          onChange={onHeadersChange}
          pairs={headers}
        />
      </TabsContent>
      <TabsContent className="min-h-0 overflow-auto" value="params">
        <KeyValueEditor noun="param" onChange={onParamsChange} pairs={params} />
      </TabsContent>
      {hasBody && (
        <TabsContent
          className="flex min-h-0 flex-col overflow-auto"
          value="body"
        >
          <Textarea
            className="flex-1 font-mono"
            onChange={(e) => onBodyChange(e.target.value)}
            placeholder="Request body"
            value={body}
          />
        </TabsContent>
      )}
    </Tabs>
  );
}
