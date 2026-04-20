interface TitleBarProps {
	dark: boolean;
	onToggleTheme: () => void;
}

export function TitleBar({ dark, onToggleTheme }: TitleBarProps) {
	return (
		<header
			className="flex shrink-0 items-center gap-2.5 border-b bg-accent pr-3.5 pl-23 py-3 select-none"
			data-tauri-drag-region
		>
			<span className="text-xs font-semibold tracking-widest text-muted-foreground">
				BRIDGE
			</span>
			<div className="flex-1" />
			<button
				type="button"
				onClick={onToggleTheme}
				className="rounded border px-2 py-0.5 text-xs tracking-wide text-muted-foreground transition-colors hover:text-foreground"
			>
				{dark ? "◑ LIGHT" : "◐ DARK"}
			</button>
		</header>
	);
}
