import {
	type HistoryItem,
	METHOD_BG_COLORS,
	METHOD_COLORS,
	statusColor,
} from "@/lib/constants";
import { cn } from "@/lib/utils";
import {
	Sidebar,
	SidebarContent,
	SidebarHeader,
	SidebarRail,
} from "./ui/sidebar";

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
		<Sidebar className="absolute h-full">
			<SidebarHeader className="p-0">
				<div className="flex shrink-0 items-center justify-between border-b px-2.5 py-2">
					<span className="text-xs tracking-widest text-muted-foreground">
						HISTORY
					</span>
					<span className="text-xs text-muted-foreground">{items.length}</span>
				</div>
			</SidebarHeader>
			<SidebarContent>
				{items.map((item, i) => (
					<SidebarRow
						key={item.ts}
						item={item}
						selected={i === selectedIndex}
						onClick={() => onSelect(i)}
					/>
				))}
			</SidebarContent>
			<SidebarRail />
		</Sidebar>
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
			type="button"
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
				<span className={cn("text-xs font-semibold", statusColor(item.status))}>
					{item.status}
				</span>
				<span className="ml-auto text-xs text-muted-foreground">{item.ts}</span>
			</div>
			<div
				className={cn(
					"truncate text-xs leading-snug",
					selected ? "text-foreground" : "text-muted-foreground",
				)}
			>
				{url}
			</div>
			<div className="mt-0.5 text-xs text-muted-foreground">{item.ms}ms</div>
		</button>
	);
}

export function MethodBadge({ method }: { method: string }) {
	return (
		<span
			className={cn(
				"inline-block min-w-11 shrink-0 rounded-sm px-1.5 py-px text-center text-xs font-semibold tracking-tight",
				METHOD_COLORS[method],
				METHOD_BG_COLORS[method],
			)}
		>
			{method}
		</span>
	);
}
