import { Cancel01Icon, PlusSignIcon } from "@hugeicons/core-free-icons";
import { HugeiconsIcon } from "@hugeicons/react";

import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";

export type Header = { key: string; value: string };

type Props = {
	headers: Header[];
	onChange: (headers: Header[]) => void;
};

export function HeadersEditor({ headers, onChange }: Props) {
	function update(i: number, patch: Partial<Header>) {
		onChange(headers.map((h, idx) => (idx === i ? { ...h, ...patch } : h)));
	}
	function remove(i: number) {
		onChange(headers.filter((_, idx) => idx !== i));
	}
	function add() {
		onChange([...headers, { key: "", value: "" }]);
	}

	return (
		<div className="flex flex-col gap-2">
			{headers.map((h, i) => (
				<div key={i} className="flex gap-2">
					<Input
						value={h.key}
						onChange={(e) => update(i, { key: e.target.value })}
						placeholder="Header"
						className="flex-1 font-mono"
					/>
					<Input
						value={h.value}
						onChange={(e) => update(i, { value: e.target.value })}
						placeholder="Value"
						className="flex-1 font-mono"
					/>
					<Button
						variant="ghost"
						size="icon"
						onClick={() => remove(i)}
						aria-label="Remove header"
					>
						<HugeiconsIcon icon={Cancel01Icon} />
					</Button>
				</div>
			))}
			<Button variant="outline" size="sm" onClick={add} className="self-start">
				<HugeiconsIcon icon={PlusSignIcon} data-icon="inline-start" />
				Add header
			</Button>
		</div>
	);
}
