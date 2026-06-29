import { invoke } from "@tauri-apps/api/core";
import { useEffect, useState } from "react";

import {
	Command,
	CommandDialog,
	CommandEmpty,
	CommandGroup,
	CommandInput,
	CommandItem,
	CommandList,
} from "@/components/ui/command";

export type HistoryEntry = {
	id: number;
	method: string;
	url: string;
	status: number;
	request_headers: [string, string][];
	request_body: string | null;
	response_headers: [string, string][];
	response_body: string;
	elapsed_ms: number;
	created_at: number;
};

function timeAgo(ms: number): string {
	const s = Math.max(0, (Date.now() - ms) / 1000);
	if (s < 60) return "just now";
	const m = s / 60;
	if (m < 60) return `${Math.floor(m)}m ago`;
	const h = m / 60;
	if (h < 24) return `${Math.floor(h)}h ago`;
	return `${Math.floor(h / 24)}d ago`;
}

type Props = {
	open: boolean;
	onOpenChange: (open: boolean) => void;
	onSelect: (entry: HistoryEntry) => void;
};

export function HistoryPalette({ open, onOpenChange, onSelect }: Props) {
	const [search, setSearch] = useState("");
	const [entries, setEntries] = useState<HistoryEntry[]>([]);

	// Debounced FTS query to the Rust backend (bm25-ranked, top results).
	useEffect(() => {
		if (!open) return;
		const handle = setTimeout(() => {
			invoke<HistoryEntry[]>("query_history", {
				query: { search: search || null, limit: 50 },
			})
				.then(setEntries)
				.catch(() => setEntries([]));
		}, 120);
		return () => clearTimeout(handle);
	}, [search, open]);

	return (
		<CommandDialog
			open={open}
			onOpenChange={onOpenChange}
			title="Search history"
			description="Search your request history"
		>
			<Command shouldFilter={false}>
				<CommandInput
					value={search}
					onValueChange={setSearch}
					placeholder="Search history…"
				/>
				<CommandList>
					<CommandEmpty>No matching requests.</CommandEmpty>
					<CommandGroup heading={search ? "Results" : "Recent"}>
						{entries.map((entry) => (
							<CommandItem
								key={entry.id}
								value={String(entry.id)}
								onSelect={() => {
									onSelect(entry);
									onOpenChange(false);
								}}
							>
								<span className="w-12 shrink-0 font-mono text-xs text-muted-foreground">
									{entry.method}
								</span>
								<span className="flex-1 truncate">{entry.url}</span>
								<span className="shrink-0 font-mono text-xs text-muted-foreground">
									{entry.status}
								</span>
								<span className="w-16 shrink-0 text-right text-xs text-muted-foreground">
									{timeAgo(entry.created_at)}
								</span>
							</CommandItem>
						))}
					</CommandGroup>
				</CommandList>
			</Command>
		</CommandDialog>
	);
}
