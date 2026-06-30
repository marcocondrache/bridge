import { useState } from "react";

type Json = null | boolean | number | string | Json[] | { [k: string]: Json };

export function JsonViewer({ data }: { data: Json }) {
  return (
    <div className="p-2 font-mono text-xs">
      <Node value={data} />
    </div>
  );
}

function Node({ value, name }: { value: Json; name?: string }) {
  const [open, setOpen] = useState(true);
  const isArray = Array.isArray(value);
  const isObject = value !== null && typeof value === "object";

  if (!isObject) {
    return (
      <div>
        {name !== undefined && <Key name={name} />}
        <Leaf value={value} />
      </div>
    );
  }

  const [open0, close] = isArray ? ["[", "]"] : ["{", "}"];
  const entries = isArray
    ? (value as Json[]).map((v, i) => [String(i), v] as const)
    : Object.entries(value as Record<string, Json>);

  return (
    <div>
      <button
        className="cursor-pointer select-none text-left hover:bg-muted"
        onClick={() => setOpen((o) => !o)}
        type="button"
      >
        {name !== undefined && <Key name={name} />}
        <span className="text-muted-foreground">
          {open ? open0 : `${open0} … ${close}`}
          {open ? "" : ` ${entries.length}`}
        </span>
      </button>
      {open && (
        <div className="border-border/50 border-l pl-4">
          {entries.map(([k, v]) => (
            <Node key={k} name={isArray ? undefined : k} value={v} />
          ))}
        </div>
      )}
      {open && <span className="text-muted-foreground">{close}</span>}
    </div>
  );
}

function Key({ name }: { name: string }) {
  return <span className="text-sky-600 dark:text-sky-400">{name}: </span>;
}

function Leaf({ value }: { value: Json }) {
  if (typeof value === "string") {
    return (
      <span className="text-emerald-600 dark:text-emerald-400">"{value}"</span>
    );
  }
  if (value === null) {
    return <span className="text-muted-foreground">null</span>;
  }
  return (
    <span className="text-amber-600 dark:text-amber-400">{String(value)}</span>
  );
}
