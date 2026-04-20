interface TitleBarProps {
	dark: boolean;
	onToggleTheme: () => void;
}

export function TitleBar({ dark, onToggleTheme }: TitleBarProps) {
	return (
		<header
			className="flex h-9.5 shrink-0 items-center gap-2.5 border-b bg-accent px-3.5 select-none"
			data-tauri-drag-region
		>
			<div className="flex gap-1.5">
				<div className="size-2.75 rounded-full bg-[#ff736a]" />
				<div className="size-2.75 rounded-full bg-[#febc2e]" />
				<div className="size-2.75 rounded-full bg-[#28c840]" />
			</div>
			<span className="ml-2.5 text-[10px] font-semibold tracking-[0.14em] text-muted-foreground">
				BRIDGE
			</span>
			<div className="flex-1" />
			<button
				type="button"
				onClick={onToggleTheme}
				className="rounded border px-2 py-0.5 text-[9px] tracking-[0.08em] text-muted-foreground transition-colors hover:text-foreground"
			>
				{dark ? "◑ LIGHT" : "◐ DARK"}
			</button>
		</header>
	);
}
