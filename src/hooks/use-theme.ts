import { useEffect } from "react";
import { useThemeStore } from "@/state/theme";

export function useTheme() {
	const { dark, toggleTheme } = useThemeStore();

	useEffect(() => {
		document.body.classList.toggle("dark", dark);
	}, [dark]);

	return { dark, toggleTheme };
}
