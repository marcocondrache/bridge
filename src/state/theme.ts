import { create } from "zustand";
import { persist } from "zustand/middleware";

type ThemeStore = {
	dark: boolean;
	toggleTheme: () => void;
};

export const useThemeStore = create<ThemeStore>()(
	persist(
		(set) => ({
			dark: false,
			toggleTheme: () => set((state: ThemeStore) => ({ dark: !state.dark })),
		}),
		{
			name: "theme",
		},
	),
);
