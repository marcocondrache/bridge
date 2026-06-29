import { Cancel01Icon, PlusSignIcon } from "@hugeicons/core-free-icons";
import { HugeiconsIcon } from "@hugeicons/react";

import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";

export interface KeyValuePair {
  key: string;
  value: string;
}

interface Props {
  noun?: string;
  onChange: (pairs: KeyValuePair[]) => void;
  pairs: KeyValuePair[];
}

export function KeyValueEditor({ pairs, onChange, noun = "entry" }: Props) {
  function update(i: number, patch: Partial<KeyValuePair>) {
    onChange(pairs.map((p, idx) => (idx === i ? { ...p, ...patch } : p)));
  }
  function remove(i: number) {
    onChange(pairs.filter((_, idx) => idx !== i));
  }
  function add() {
    onChange([...pairs, { key: "", value: "" }]);
  }

  const Noun = noun.charAt(0).toUpperCase() + noun.slice(1);

  return (
    <div className="flex flex-col gap-2">
      {pairs.map((p, i) => (
        // biome-ignore lint/suspicious/noArrayIndexKey: stable key
        <div className="flex gap-2" key={i}>
          <Input
            className="flex-1 font-mono"
            onChange={(e) => update(i, { key: e.target.value })}
            placeholder={Noun}
            value={p.key}
          />
          <Input
            className="flex-1 font-mono"
            onChange={(e) => update(i, { value: e.target.value })}
            placeholder="Value"
            value={p.value}
          />
          <Button
            aria-label={`Remove ${noun}`}
            onClick={() => remove(i)}
            size="icon"
            variant="ghost"
          >
            <HugeiconsIcon icon={Cancel01Icon} />
          </Button>
        </div>
      ))}
      <Button className="self-start" onClick={add} size="sm" variant="outline">
        <HugeiconsIcon data-icon="inline-start" icon={PlusSignIcon} />
        Add {noun}
      </Button>
    </div>
  );
}
