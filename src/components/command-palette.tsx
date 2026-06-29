import {
	Command,
	CommandDialog,
	CommandEmpty,
	CommandGroup,
	CommandInput,
	CommandItem,
	CommandList,
	CommandShortcut,
} from "@/components/ui/command";

export type PaletteCommand = {
	id: string;
	label: string;
	shortcut?: string;
	disabled?: boolean;
	run: () => void;
};

type Props = {
	open: boolean;
	onOpenChange: (open: boolean) => void;
	commands: PaletteCommand[];
};

export function CommandPalette({ open, onOpenChange, commands }: Props) {
	return (
		<CommandDialog
			open={open}
			onOpenChange={onOpenChange}
			title="Commands"
			description="Run a command"
		>
			<Command>
				<CommandInput placeholder="Type a command…" />
				<CommandList>
					<CommandEmpty>No commands found.</CommandEmpty>
					<CommandGroup heading="Commands">
						{commands.map((cmd) => (
							<CommandItem
								key={cmd.id}
								value={cmd.label}
								disabled={cmd.disabled}
								onSelect={() => {
									onOpenChange(false);
									cmd.run();
								}}
							>
								<span className="flex-1">{cmd.label}</span>
								{cmd.shortcut && (
									<CommandShortcut>{cmd.shortcut}</CommandShortcut>
								)}
							</CommandItem>
						))}
					</CommandGroup>
				</CommandList>
			</Command>
		</CommandDialog>
	);
}
