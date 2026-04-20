import { type HttpMethod } from "@/lib/constants";
import { Input } from "@/components/ui/input";
import { Button } from "@/components/ui/button";
import { Kbd } from "@/components/ui/kbd";
import { MethodDropdown } from "./method-dropdown";

interface UrlBarProps {
  url: string;
  method: HttpMethod;
  onUrlChange: (url: string) => void;
  onMethodChange: (method: HttpMethod) => void;
  onSend: () => void;
}

export function UrlBar({
  url,
  method,
  onUrlChange,
  onMethodChange,
  onSend,
}: UrlBarProps) {
  return (
    <div className="flex shrink-0 items-center gap-2 border-b bg-accent px-3 py-1.5">
      <MethodDropdown method={method} onChange={onMethodChange} />
      <Input
        value={url}
        onChange={(e) => onUrlChange(e.target.value)}
        onKeyDown={(e) => {
          if (e.key === "Enter" && (e.metaKey || e.ctrlKey)) {
            onSend();
          }
        }}
        placeholder="https://..."
        spellCheck={false}
        className="flex-1 text-xs"
      />
      <Button
        size="sm"
        onClick={onSend}
        className="shrink-0 px-3.5 text-[10px] font-semibold tracking-[0.06em]"
      >
        SEND
      </Button>
      <Kbd>⏎</Kbd>
    </div>
  );
}
