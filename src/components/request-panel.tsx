import { REQ_HEADERS, REQ_PARAMS } from "@/lib/constants";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";
import { KVTable } from "./kv-table";

export function RequestPanel() {
  return (
    <Tabs defaultValue="headers" className="flex min-w-0 flex-1 flex-col overflow-hidden">
      <div className="flex shrink-0 items-center border-b bg-accent pl-2.5">
        <span className="mr-2.5 text-[9px] tracking-[0.12em] text-muted-foreground">
          REQUEST
        </span>
        <TabsList variant="line">
          <TabsTrigger value="params">Params</TabsTrigger>
          <TabsTrigger value="headers">Headers</TabsTrigger>
          <TabsTrigger value="body">Body</TabsTrigger>
          <TabsTrigger value="auth">Auth</TabsTrigger>
        </TabsList>
      </div>
      <div className="flex-1 overflow-y-auto">
        <TabsContent value="params">
          <KVTable rows={REQ_PARAMS} />
        </TabsContent>
        <TabsContent value="headers">
          <KVTable rows={REQ_HEADERS} />
        </TabsContent>
        <TabsContent value="body">
          <div className="p-4 text-center text-[10px] text-muted-foreground">
            No body
          </div>
        </TabsContent>
        <TabsContent value="auth">
          <div className="p-4 text-center text-[10px] text-muted-foreground">
            No auth configured
          </div>
        </TabsContent>
      </div>
    </Tabs>
  );
}
