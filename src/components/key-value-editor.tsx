import { Cancel01Icon, PlusSignIcon } from "@hugeicons/core-free-icons";
import { HugeiconsIcon } from "@hugeicons/react";

import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";

export type KeyValuePair = { key: string; value: string };

type Props = {
	pairs: KeyValuePair[];
	onChange: (pairs: KeyValuePair[]) => void;
	noun?: string;
};

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
				<div key={i} className="flex gap-2">
					<Input
						value={p.key}
						onChange={(e) => update(i, { key: e.target.value })}
						placeholder={Noun}
						className="flex-1 font-mono"
					/>
					<Input
						value={p.value}
						onChange={(e) => update(i, { value: e.target.value })}
						placeholder="Value"
						className="flex-1 font-mono"
					/>
					<Button
						variant="ghost"
						size="icon"
						onClick={() => remove(i)}
						aria-label={`Remove ${noun}`}
					>
						<HugeiconsIcon icon={Cancel01Icon} />
					</Button>
				</div>
			))}
			<Button variant="outline" size="sm" onClick={add} className="self-start">
				<HugeiconsIcon icon={PlusSignIcon} data-icon="inline-start" />
				Add {noun}
			</Button>
		</div>
	);
}
