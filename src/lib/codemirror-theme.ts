import { HighlightStyle, syntaxHighlighting } from "@codemirror/language";
import { EditorView } from "@codemirror/view";
import { tags } from "@lezer/highlight";

const editorTheme = EditorView.theme({
	"&": {
		backgroundColor: "var(--background)",
		color: "var(--foreground)",
	},
	".cm-content": {
		caretColor: "var(--foreground)",
	},
	".cm-cursor, .cm-dropCursor": {
		borderLeftColor: "var(--foreground)",
	},
	"&.cm-focused .cm-selectionBackground, .cm-selectionBackground, .cm-content ::selection":
		{
			backgroundColor: "var(--accent)",
		},
	".cm-panels": {
		backgroundColor: "var(--card)",
		color: "var(--card-foreground)",
	},
	".cm-panels.cm-panels-top": {
		borderBottom: "1px solid var(--border)",
	},
	".cm-panels.cm-panels-bottom": {
		borderTop: "1px solid var(--border)",
	},
	".cm-searchMatch": {
		backgroundColor: "var(--accent)",
	},
	".cm-searchMatch.cm-searchMatch-selected": {
		backgroundColor: "var(--accent)",
	},
	".cm-activeLine": {
		backgroundColor: "var(--accent)",
	},
	".cm-selectionMatch": {
		backgroundColor: "var(--accent)",
	},
	"&.cm-focused .cm-matchingBracket, &.cm-focused .cm-nonmatchingBracket": {
		backgroundColor: "var(--accent)",
	},
	".cm-gutters": {
		backgroundColor: "var(--background)",
		color: "var(--muted-foreground)",
		borderRight: "1px solid var(--border)",
	},
	".cm-activeLineGutter": {
		backgroundColor: "var(--accent)",
	},
	".cm-foldPlaceholder": {
		backgroundColor: "var(--muted)",
		color: "var(--muted-foreground)",
		border: "none",
	},
	".cm-tooltip": {
		backgroundColor: "var(--popover)",
		color: "var(--popover-foreground)",
		border: "1px solid var(--border)",
	},
	".cm-tooltip .cm-tooltip-arrow:before": {
		borderTopColor: "var(--border)",
		borderBottomColor: "var(--border)",
	},
	".cm-tooltip .cm-tooltip-arrow:after": {
		borderTopColor: "var(--popover)",
		borderBottomColor: "var(--popover)",
	},
	".cm-tooltip-autocomplete": {
		"& > ul > li[aria-selected]": {
			backgroundColor: "var(--accent)",
			color: "var(--accent-foreground)",
		},
	},
});

const highlightStyle = HighlightStyle.define([
	{ tag: tags.keyword, color: "var(--chart-1)" },
	{ tag: tags.operator, color: "var(--muted-foreground)" },
	{ tag: tags.special(tags.variableName), color: "var(--chart-2)" },
	{ tag: tags.typeName, color: "var(--chart-3)" },
	{ tag: tags.atom, color: "var(--chart-4)" },
	{ tag: tags.number, color: "var(--chart-4)" },
	{ tag: tags.bool, color: "var(--chart-4)" },
	{ tag: tags.null, color: "var(--muted-foreground)" },
	{ tag: tags.definition(tags.variableName), color: "var(--chart-2)" },
	{ tag: tags.string, color: "var(--chart-5)" },
	{ tag: tags.special(tags.string), color: "var(--chart-5)" },
	{ tag: tags.comment, color: "var(--muted-foreground)" },
	{ tag: tags.variableName, color: "var(--foreground)" },
	{ tag: tags.bracket, color: "var(--muted-foreground)" },
	{ tag: tags.tagName, color: "var(--chart-1)" },
	{ tag: tags.attributeName, color: "var(--chart-2)" },
	{ tag: tags.propertyName, color: "var(--chart-1)" },
	{ tag: tags.punctuation, color: "var(--muted-foreground)" },
	{ tag: tags.separator, color: "var(--muted-foreground)" },
]);

export const shadcnTheme = [editorTheme, syntaxHighlighting(highlightStyle)];
