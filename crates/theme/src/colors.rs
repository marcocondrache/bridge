use gpui::{Hsla, hsla};

use crate::scales::*;

#[derive(Debug, Default, Clone, Copy)]
pub struct ThemeColors {
    /// Used for accents such as hover background on MenuItem, ListItem, etc.
    pub accent: Hsla,
    /// Used for accent text color.
    pub accent_foreground: Hsla,
    /// Accordion background color.
    pub accordion: Hsla,
    /// Accordion hover background color.
    pub accordion_hover: Hsla,
    /// Default background color.
    pub background: Hsla,
    /// Default border color
    pub border: Hsla,
    /// Background color for GroupBox.
    pub group_box: Hsla,
    /// Text color for GroupBox.
    pub group_box_foreground: Hsla,
    /// Input caret color (Blinking cursor).
    pub caret: Hsla,
    /// Danger background color.
    pub danger: Hsla,
    /// Danger active background color.
    pub danger_active: Hsla,
    /// Danger text color.
    pub danger_foreground: Hsla,
    /// Danger hover background color.
    pub danger_hover: Hsla,
    /// Description List label background color.
    pub description_list_label: Hsla,
    /// Description List label foreground color.
    pub description_list_label_foreground: Hsla,
    /// Drag border color.
    pub drag_border: Hsla,
    /// Drop target background color.
    pub drop_target: Hsla,
    /// Default text color.
    pub foreground: Hsla,
    /// Info background color.
    pub info: Hsla,
    /// Info active background color.
    pub info_active: Hsla,
    /// Info text color.
    pub info_foreground: Hsla,
    /// Info hover background color.
    pub info_hover: Hsla,
    /// Border color for inputs such as Input, Dropdown, etc.
    pub input: Hsla,
    /// Link text color.
    pub link: Hsla,
    /// Active link text color.
    pub link_active: Hsla,
    /// Hover link text color.
    pub link_hover: Hsla,
    /// Background color for List and ListItem.
    pub list: Hsla,
    /// Background color for active ListItem.
    pub list_active: Hsla,
    /// Border color for active ListItem.
    pub list_active_border: Hsla,
    /// Stripe background color for even ListItem.
    pub list_even: Hsla,
    /// Background color for List header.
    pub list_head: Hsla,
    /// Hover background color for ListItem.
    pub list_hover: Hsla,
    /// Muted backgrounds such as Skeleton and Switch.
    pub muted: Hsla,
    /// Muted text color, as used in disabled text.
    pub muted_foreground: Hsla,
    /// Background color for Popover.
    pub popover: Hsla,
    /// Text color for Popover.
    pub popover_foreground: Hsla,
    /// Primary background color.
    pub primary: Hsla,
    /// Active primary background color.
    pub primary_active: Hsla,
    /// Primary text color.
    pub primary_foreground: Hsla,
    /// Hover primary background color.
    pub primary_hover: Hsla,
    /// Progress bar background color.
    pub progress_bar: Hsla,
    /// Used for focus ring.
    pub ring: Hsla,
    /// Scrollbar background color.
    pub scrollbar: Hsla,
    /// Scrollbar thumb background color.
    pub scrollbar_thumb: Hsla,
    /// Scrollbar thumb hover background color.
    pub scrollbar_thumb_hover: Hsla,
    /// Secondary background color.
    pub secondary: Hsla,
    /// Active secondary background color.
    pub secondary_active: Hsla,
    /// Secondary text color, used for secondary Button text color or secondary text.
    pub secondary_foreground: Hsla,
    /// Hover secondary background color.
    pub secondary_hover: Hsla,
    /// Input selection background color.
    pub selection: Hsla,
    /// Sidebar background color.
    pub sidebar: Hsla,
    /// Sidebar accent background color.
    pub sidebar_accent: Hsla,
    /// Sidebar accent text color.
    pub sidebar_accent_foreground: Hsla,
    /// Sidebar border color.
    pub sidebar_border: Hsla,
    /// Sidebar text color.
    pub sidebar_foreground: Hsla,
    /// Sidebar primary background color.
    pub sidebar_primary: Hsla,
    /// Sidebar primary text color.
    pub sidebar_primary_foreground: Hsla,
    /// Skeleton background color.
    pub skeleton: Hsla,
    /// Slider bar background color.
    pub slider_bar: Hsla,
    /// Slider thumb background color.
    pub slider_thumb: Hsla,
    /// Success background color.
    pub success: Hsla,
    /// Success text color.
    pub success_foreground: Hsla,
    /// Success hover background color.
    pub success_hover: Hsla,
    /// Success active background color.
    pub success_active: Hsla,
    /// Switch background color.
    pub switch: Hsla,
    /// Tab background color.
    pub tab: Hsla,
    /// Tab active background color.
    pub tab_active: Hsla,
    /// Tab active text color.
    pub tab_active_foreground: Hsla,
    /// TabBar background color.
    pub tab_bar: Hsla,
    /// TabBar segmented background color.
    pub tab_bar_segmented: Hsla,
    /// Tab text color.
    pub tab_foreground: Hsla,
    /// Table background color.
    pub table: Hsla,
    /// Table active item background color.
    pub table_active: Hsla,
    /// Table active item border color.
    pub table_active_border: Hsla,
    /// Stripe background color for even TableRow.
    pub table_even: Hsla,
    /// Table head background color.
    pub table_head: Hsla,
    /// Table head text color.
    pub table_head_foreground: Hsla,
    /// Table item hover background color.
    pub table_hover: Hsla,
    /// Table row border color.
    pub table_row_border: Hsla,
    /// TitleBar background color, use for Window title bar.
    pub title_bar: Hsla,
    /// TitleBar border color.
    pub title_bar_border: Hsla,
    /// Background color for Tiles.
    pub tiles: Hsla,
    /// Warning background color.
    pub warning: Hsla,
    /// Warning active background color.
    pub warning_active: Hsla,
    /// Warning hover background color.
    pub warning_hover: Hsla,
    /// Warning foreground color.
    pub warning_foreground: Hsla,
    /// Overlay background color.
    pub overlay: Hsla,
    /// Window border color.
    ///
    /// # Platform specific:
    ///
    /// This is only works on Linux, other platforms we can't change the window border color.
    pub window_border: Hsla,
}

impl ThemeColors {
    pub fn light() -> Self {
        let zinc = zinc();
        let blue = blue();
        let green = green();
        let red = red();
        let amber = amber();

        Self {
            // Accent colors - using blue for light theme
            accent: blue.shade_100,
            accent_foreground: blue.shade_900,

            // Accordion
            accordion: zinc.shade_50,
            accordion_hover: zinc.shade_100,

            // Background and border
            background: gpui::white(),
            border: zinc.shade_200,

            // Group box
            group_box: zinc.shade_50,
            group_box_foreground: zinc.shade_900,

            // Caret
            caret: zinc.shade_950,

            // Danger/destructive colors
            danger: red.shade_500,
            danger_active: red.shade_700,
            danger_foreground: gpui::white(),
            danger_hover: red.shade_600,

            // Description list
            description_list_label: zinc.shade_100,
            description_list_label_foreground: zinc.shade_700,

            // Drag and drop
            drag_border: blue.shade_400,
            drop_target: blue.shade_100,

            // Foreground
            foreground: zinc.shade_950,

            // Info colors
            info: blue.shade_500,
            info_active: blue.shade_700,
            info_foreground: gpui::white(),
            info_hover: blue.shade_600,

            // Input
            input: zinc.shade_300,

            // Link
            link: blue.shade_600,
            link_active: blue.shade_800,
            link_hover: blue.shade_700,

            // List
            list: gpui::white(),
            list_active: blue.shade_100,
            list_active_border: blue.shade_500,
            list_even: zinc.shade_50,
            list_head: zinc.shade_100,
            list_hover: zinc.shade_100,

            // Muted
            muted: zinc.shade_100,
            muted_foreground: zinc.shade_500,

            // Popover
            popover: gpui::white(),
            popover_foreground: zinc.shade_950,

            // Primary
            primary: zinc.shade_900,
            primary_active: zinc.shade_950,
            primary_foreground: zinc.shade_50,
            primary_hover: zinc.shade_800,

            // Progress bar
            progress_bar: blue.shade_600,

            // Ring (focus)
            ring: blue.shade_500,

            // Scrollbar
            scrollbar: zinc.shade_100,
            scrollbar_thumb: zinc.shade_300,
            scrollbar_thumb_hover: zinc.shade_400,

            // Secondary
            secondary: zinc.shade_100,
            secondary_active: zinc.shade_300,
            secondary_foreground: zinc.shade_900,
            secondary_hover: zinc.shade_200,

            // Selection
            selection: blue.shade_200,

            // Sidebar
            sidebar: zinc.shade_50,
            sidebar_accent: blue.shade_100,
            sidebar_accent_foreground: blue.shade_900,
            sidebar_border: zinc.shade_200,
            sidebar_foreground: zinc.shade_700,
            sidebar_primary: blue.shade_600,
            sidebar_primary_foreground: gpui::white(),

            // Skeleton
            skeleton: zinc.shade_200,

            // Slider
            slider_bar: zinc.shade_200,
            slider_thumb: blue.shade_600,

            // Success
            success: green.shade_500,
            success_foreground: gpui::white(),
            success_hover: green.shade_600,
            success_active: green.shade_700,

            // Switch
            switch: zinc.shade_200,

            // Tab
            tab: zinc.shade_100,
            tab_active: gpui::white(),
            tab_active_foreground: zinc.shade_950,
            tab_bar: zinc.shade_50,
            tab_bar_segmented: zinc.shade_100,
            tab_foreground: zinc.shade_600,

            // Table
            table: gpui::white(),
            table_active: blue.shade_100,
            table_active_border: blue.shade_500,
            table_even: zinc.shade_50,
            table_head: zinc.shade_100,
            table_head_foreground: zinc.shade_900,
            table_hover: zinc.shade_100,
            table_row_border: zinc.shade_200,

            // Title bar
            title_bar: zinc.shade_50,
            title_bar_border: zinc.shade_200,

            // Tiles
            tiles: zinc.shade_50,

            // Warning
            warning: amber.shade_500,
            warning_active: amber.shade_700,
            warning_hover: amber.shade_600,
            warning_foreground: gpui::white(),

            // Overlay
            overlay: hsla(0.0, 0.0, 0.0, 0.5), // semi-transparent black

            // Window border
            window_border: zinc.shade_300,
        }
    }

    pub fn dark() -> Self {
        let zinc = zinc();
        let blue = blue();
        let green = green();
        let red = red();
        let amber = amber();

        Self {
            // Accent colors - using blue for dark theme
            accent: blue.shade_900,
            accent_foreground: blue.shade_100,

            // Accordion
            accordion: zinc.shade_900,
            accordion_hover: zinc.shade_800,

            // Background and border
            background: zinc.shade_950,
            border: zinc.shade_800,

            // Group box
            group_box: zinc.shade_900,
            group_box_foreground: zinc.shade_100,

            // Caret
            caret: zinc.shade_50,

            // Danger/destructive colors
            danger: red.shade_600,
            danger_active: red.shade_800,
            danger_foreground: zinc.shade_50,
            danger_hover: red.shade_700,

            // Description list
            description_list_label: zinc.shade_800,
            description_list_label_foreground: zinc.shade_300,

            // Drag and drop
            drag_border: blue.shade_500,
            drop_target: blue.shade_950,

            // Foreground
            foreground: zinc.shade_50,

            // Info colors
            info: blue.shade_600,
            info_active: blue.shade_800,
            info_foreground: zinc.shade_50,
            info_hover: blue.shade_700,

            // Input
            input: zinc.shade_700,

            // Link
            link: blue.shade_400,
            link_active: blue.shade_200,
            link_hover: blue.shade_300,

            // List
            list: zinc.shade_950,
            list_active: blue.shade_950,
            list_active_border: blue.shade_500,
            list_even: zinc.shade_900,
            list_head: zinc.shade_900,
            list_hover: zinc.shade_900,

            // Muted
            muted: zinc.shade_800,
            muted_foreground: zinc.shade_500,

            // Popover
            popover: zinc.shade_900,
            popover_foreground: zinc.shade_50,

            // Primary
            primary: zinc.shade_50,
            primary_active: zinc.shade_200,
            primary_foreground: zinc.shade_950,
            primary_hover: zinc.shade_100,

            // Progress bar
            progress_bar: blue.shade_500,

            // Ring (focus)
            ring: blue.shade_400,

            // Scrollbar
            scrollbar: zinc.shade_900,
            scrollbar_thumb: zinc.shade_700,
            scrollbar_thumb_hover: zinc.shade_600,

            // Secondary
            secondary: zinc.shade_800,
            secondary_active: zinc.shade_600,
            secondary_foreground: zinc.shade_100,
            secondary_hover: zinc.shade_700,

            // Selection
            selection: blue.shade_900,

            // Sidebar
            sidebar: zinc.shade_900,
            sidebar_accent: blue.shade_900,
            sidebar_accent_foreground: blue.shade_100,
            sidebar_border: zinc.shade_800,
            sidebar_foreground: zinc.shade_400,
            sidebar_primary: blue.shade_500,
            sidebar_primary_foreground: zinc.shade_50,

            // Skeleton
            skeleton: zinc.shade_800,

            // Slider
            slider_bar: zinc.shade_700,
            slider_thumb: blue.shade_500,

            // Success
            success: green.shade_600,
            success_foreground: zinc.shade_50,
            success_hover: green.shade_700,
            success_active: green.shade_800,

            // Switch
            switch: zinc.shade_700,

            // Tab
            tab: zinc.shade_900,
            tab_active: zinc.shade_800,
            tab_active_foreground: zinc.shade_50,
            tab_bar: zinc.shade_950,
            tab_bar_segmented: zinc.shade_900,
            tab_foreground: zinc.shade_400,

            // Table
            table: zinc.shade_950,
            table_active: blue.shade_950,
            table_active_border: blue.shade_500,
            table_even: zinc.shade_900,
            table_head: zinc.shade_900,
            table_head_foreground: zinc.shade_100,
            table_hover: zinc.shade_900,
            table_row_border: zinc.shade_800,

            // Title bar
            title_bar: zinc.shade_900,
            title_bar_border: zinc.shade_800,

            // Tiles
            tiles: zinc.shade_900,

            // Warning
            warning: amber.shade_600,
            warning_active: amber.shade_800,
            warning_hover: amber.shade_700,
            warning_foreground: zinc.shade_50,

            // Overlay
            overlay: hsla(0.0, 0.0, 0.0, 0.7), // semi-transparent black (darker for dark theme)

            // Window border
            window_border: zinc.shade_700,
        }
    }
}
