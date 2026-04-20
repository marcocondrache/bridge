import {
	type HistoryItem,
	METHOD_BG_COLORS,
	METHOD_COLORS,
	statusColor,
} from "@/lib/constants";
import { cn } from "@/lib/utils";

interface HistorySidebarProps {
	items: HistoryItem[];
	selectedIndex: number;
	onSelect: (index: number) => void;
}

export function HistorySidebar({
	items,
	selectedIndex,
	onSelect,
}: HistorySidebarProps) {
	return (
		<aside className="flex w-[210px] shrink-0 flex-col overflow-hidden border-r bg-accent">
			<div className="flex shrink-0 items-center justify-between border-b px-2.5 py-1.5">
				<span className="text-[9px] tracking-[0.12em] text-muted-foreground">
					HISTORY
				</span>
				<span className="text-[9px] text-muted-foreground">{items.length}</span>
			</div>
			<div className="flex-1 overflow-y-auto">
				{items.map((item, i) => (
					<SidebarRow
						key={i}
						item={item}
						selected={i === selectedIndex}
						onClick={() => onSelect(i)}
					/>
				))}
			</div>
		</aside>
	);
}

function SidebarRow({
	item,
	selected,
	onClick,
}: {
	item: HistoryItem;
	selected: boolean;
	onClick: () => void;
}) {
	const url = item.url.replace(/^https?:\/\//, "");

	return (
		<button
			onClick={onClick}
			className={cn(
				"w-full border-l-2 px-2.5 py-1.5 text-left transition-colors",
				selected
					? "border-l-primary bg-muted"
					: "border-l-transparent hover:bg-muted/50",
			)}
		>
			<div className="mb-0.5 flex items-center gap-1.5">
				<MethodBadge method={item.method} />
				<span
					className={cn("text-[9px] font-semibold", statusColor(item.status))}
				>
					{item.status}
				</span>
				<span className="ml-auto text-[9px] text-muted-foreground">
					{item.ts}
				</span>
			</div>
			<div
				className={cn(
					"truncate text-[10px] leading-snug",
					selected ? "text-foreground" : "text-muted-foreground",
				)}
			>
				{url}
			</div>
			<div className="mt-0.5 text-[9px] text-muted-foreground">{item.ms}ms</div>
		</button>
	);
}

export function MethodBadge({ method }: { method: string }) {
	return (
		<span
			className={cn(
				"inline-block min-w-[46px] shrink-0 rounded-[3px] px-1.5 py-px text-center text-[10px] font-semibold tracking-[0.04em]",
				METHOD_COLORS[method],
				METHOD_BG_COLORS[method],
			)}
		>
			{method}
		</span>
	);
}
