import { Kbd, KbdGroup } from "./ui/kbd";

function KbdHint({ keys, label }: { keys: string[]; label: string }) {
  return (
    <div className="flex items-center gap-1">
      <KbdGroup>
        {keys.map((k) => (
          <Kbd key={k}>{k}</Kbd>
        ))}
      </KbdGroup>
      <span className="text-[8px] text-muted-foreground">{label}</span>
    </div>
  );
}

export function StatusBar() {
  return (
    <footer className="flex h-5.5 shrink-0 items-center gap-3.5 border-t bg-accent px-3">
      <KbdHint keys={["⌘", "K"]} label="palette" />
      <KbdHint keys={["⌘", "/"]} label="shortcuts" />
      <KbdHint keys={["⌘", "⏎"]} label="send" />
      <KbdHint keys={["⌘", "↑↓"]} label="history" />
      <div className="flex-1" />
      <span className="text-[8px] tracking-widest text-muted-foreground">
        BRIDGE v0.1.0
      </span>
    </footer>
  );
}
