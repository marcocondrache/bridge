import type { KVRow } from "@/lib/constants";
import { cn } from "@/lib/utils";

interface KVTableProps {
	rows: KVRow[];
	readOnly?: boolean;
}

export function KVTable({ rows, readOnly = false }: KVTableProps) {
	return (
		<table className="w-full border-collapse text-xs">
			<tbody>
				{rows.map((r, i) => (
					<tr key={i} className="border-b border-border/30">
						{!readOnly && (
							<td className="w-5.5 pl-2.5">
								<div
									className={cn(
										"size-2 rounded-sm",
										r.on ? "bg-primary" : "bg-border",
									)}
								/>
							</td>
						)}
						<td
							className={cn(
								"w-1/3 max-w-0 truncate py-1",
								readOnly ? "pl-3.5 pr-2" : "px-1.5",
								r.on ? "text-foreground" : "text-muted-foreground",
							)}
						>
							{r.key}
						</td>
						<td
							className={cn(
								"max-w-0 truncate py-1 pr-3 pl-1.5",
								r.on ? "text-muted-foreground" : "text-muted-foreground/60",
							)}
						>
							{r.value}
						</td>
					</tr>
				))}
				{!readOnly && (
					<tr>
						<td colSpan={3} className="px-2.5 py-1.5">
							<span className="text-xs text-muted-foreground">
								+ add header
							</span>
						</td>
					</tr>
				)}
			</tbody>
		</table>
	);
}
