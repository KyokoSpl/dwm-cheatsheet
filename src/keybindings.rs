use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Keybinding {
    pub modifiers: Vec<String>,
    pub key: String,
    pub function: String,
    pub description: String,
    pub category: Category,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum Category {
    Media,
    Screenshot,
    Applications,
    WindowManagement,
    Layout,
    Gaps,
    Navigation,
    Tags,
    System,
    Borders,
}

impl Category {
    pub fn as_str(&self) -> &'static str {
        match self {
            Category::Media => "Media Controls",
            Category::Screenshot => "Screenshots",
            Category::Applications => "Applications",
            Category::WindowManagement => "Window Management",
            Category::Layout => "Layout Management",
            Category::Gaps => "Gap Management",
            Category::Navigation => "Navigation",
            Category::Tags => "Tags & Workspaces",
            Category::System => "System",
            Category::Borders => "Borders",
        }
    }

    pub fn color(&self) -> &'static str {
        match self {
            Category::Media => "#f38ba8",          // Red
            Category::Screenshot => "#fab387",      // Orange
            Category::Applications => "#f9e2af",    // Yellow
            Category::WindowManagement => "#a6e3a1", // Green
            Category::Layout => "#94e2d5",          // Teal
            Category::Gaps => "#89b4fa",            // Blue
            Category::Navigation => "#cba6f7",      // Purple
            Category::Tags => "#f5c2e7",           // Pink
            Category::System => "#eba0ac",          // Maroon
            Category::Borders => "#9399b2",         // Overlay1
        }
    }
}

pub fn get_keybindings() -> Vec<Keybinding> {
    vec![
        // Media Controls
        Keybinding {
            modifiers: vec![],
            key: "XF86AudioLowerVolume".to_string(),
            function: "Volume Down".to_string(),
            description: "Decrease volume by 5%".to_string(),
            category: Category::Media,
        },
        Keybinding {
            modifiers: vec![],
            key: "XF86AudioRaiseVolume".to_string(),
            function: "Volume Up".to_string(),
            description: "Increase volume by 5%".to_string(),
            category: Category::Media,
        },
        Keybinding {
            modifiers: vec![],
            key: "XF86AudioMute".to_string(),
            function: "Mute Toggle".to_string(),
            description: "Toggle audio mute".to_string(),
            category: Category::Media,
        },
        Keybinding {
            modifiers: vec![],
            key: "XF86MonBrightnessUp".to_string(),
            function: "Brightness Up".to_string(),
            description: "Increase screen brightness".to_string(),
            category: Category::Media,
        },
        Keybinding {
            modifiers: vec![],
            key: "XF86MonBrightnessDown".to_string(),
            function: "Brightness Down".to_string(),
            description: "Decrease screen brightness".to_string(),
            category: Category::Media,
        },
        Keybinding {
            modifiers: vec![],
            key: "XF86AudioPlay".to_string(),
            function: "Play/Pause".to_string(),
            description: "Toggle media playback".to_string(),
            category: Category::Media,
        },
        Keybinding {
            modifiers: vec![],
            key: "XF86AudioNext".to_string(),
            function: "Next Track".to_string(),
            description: "Skip to next track".to_string(),
            category: Category::Media,
        },
        Keybinding {
            modifiers: vec![],
            key: "XF86AudioPrev".to_string(),
            function: "Previous Track".to_string(),
            description: "Go to previous track".to_string(),
            category: Category::Media,
        },

        // Screenshots
        Keybinding {
            modifiers: vec!["Super".to_string(), "Ctrl".to_string()],
            key: "u".to_string(),
            function: "Screenshot (Full)".to_string(),
            description: "Take fullscreen screenshot to clipboard".to_string(),
            category: Category::Screenshot,
        },
        Keybinding {
            modifiers: vec!["Super".to_string()],
            key: "u".to_string(),
            function: "Screenshot (Select)".to_string(),
            description: "Take selected area screenshot to clipboard".to_string(),
            category: Category::Screenshot,
        },
        Keybinding {
            modifiers: vec!["Super".to_string(), "Alt".to_string()],
            key: "s".to_string(),
            function: "Screenshot Menu".to_string(),
            description: "Open screenshot utility menu".to_string(),
            category: Category::Screenshot,
        },

        // Applications
        Keybinding {
            modifiers: vec!["Super".to_string()],
            key: "b".to_string(),
            function: "Browser".to_string(),
            description: "Launch Firefox browser".to_string(),
            category: Category::Applications,
        },
        Keybinding {
            modifiers: vec!["Super".to_string()],
            key: "e".to_string(),
            function: "File Manager".to_string(),
            description: "Launch PCManFM file manager".to_string(),
            category: Category::Applications,
        },
        Keybinding {
            modifiers: vec!["Super".to_string(), "Shift".to_string()],
            key: "e".to_string(),
            function: "Code Editor".to_string(),
            description: "Launch VS Code".to_string(),
            category: Category::Applications,
        },
        Keybinding {
            modifiers: vec!["Super".to_string()],
            key: "Return".to_string(),
            function: "Terminal".to_string(),
            description: "Launch Kitty terminal".to_string(),
            category: Category::Applications,
        },
        Keybinding {
            modifiers: vec!["Super".to_string(), "Shift".to_string()],
            key: "Return".to_string(),
            function: "App Launcher".to_string(),
            description: "Open Rofi application launcher".to_string(),
            category: Category::Applications,
        },
        Keybinding {
            modifiers: vec!["Super".to_string(), "Alt".to_string()],
            key: "e".to_string(),
            function: "EWW Widget".to_string(),
            description: "Open EWW widget".to_string(),
            category: Category::Applications,
        },

        // Window Management
        Keybinding {
            modifiers: vec!["Super".to_string()],
            key: "c".to_string(),
            function: "Kill Window".to_string(),
            description: "Close the focused window".to_string(),
            category: Category::WindowManagement,
        },
        Keybinding {
            modifiers: vec!["Super".to_string(), "Shift".to_string()],
            key: "Space".to_string(),
            function: "Toggle Floating".to_string(),
            description: "Toggle floating mode for focused window".to_string(),
            category: Category::WindowManagement,
        },
        Keybinding {
            modifiers: vec!["Super".to_string()],
            key: "f".to_string(),
            function: "Toggle Fullscreen".to_string(),
            description: "Toggle fullscreen for focused window".to_string(),
            category: Category::WindowManagement,
        },
        Keybinding {
            modifiers: vec!["Super".to_string()],
            key: "n".to_string(),
            function: "Hide Window".to_string(),
            description: "Hide the focused window".to_string(),
            category: Category::WindowManagement,
        },
        Keybinding {
            modifiers: vec!["Super".to_string(), "Shift".to_string()],
            key: "n".to_string(),
            function: "Restore Window".to_string(),
            description: "Restore hidden window".to_string(),
            category: Category::WindowManagement,
        },
        Keybinding {
            modifiers: vec!["Super".to_string(), "Shift".to_string()],
            key: "Return".to_string(),
            function: "Zoom Master".to_string(),
            description: "Move window to/from master area".to_string(),
            category: Category::WindowManagement,
        },

        // Navigation
        Keybinding {
            modifiers: vec!["Super".to_string()],
            key: "j".to_string(),
            function: "Focus Next".to_string(),
            description: "Focus next window in stack".to_string(),
            category: Category::Navigation,
        },
        Keybinding {
            modifiers: vec!["Super".to_string()],
            key: "k".to_string(),
            function: "Focus Previous".to_string(),
            description: "Focus previous window in stack".to_string(),
            category: Category::Navigation,
        },
        Keybinding {
            modifiers: vec!["Super".to_string(), "Shift".to_string()],
            key: "j".to_string(),
            function: "Move Down".to_string(),
            description: "Move window down in stack".to_string(),
            category: Category::Navigation,
        },
        Keybinding {
            modifiers: vec!["Super".to_string(), "Shift".to_string()],
            key: "k".to_string(),
            function: "Move Up".to_string(),
            description: "Move window up in stack".to_string(),
            category: Category::Navigation,
        },
        Keybinding {
            modifiers: vec!["Super".to_string()],
            key: "Left".to_string(),
            function: "Previous Tag".to_string(),
            description: "Switch to previous tag".to_string(),
            category: Category::Navigation,
        },
        Keybinding {
            modifiers: vec!["Super".to_string()],
            key: "Right".to_string(),
            function: "Next Tag".to_string(),
            description: "Switch to next tag".to_string(),
            category: Category::Navigation,
        },
        Keybinding {
            modifiers: vec!["Super".to_string()],
            key: "Tab".to_string(),
            function: "Last Tag".to_string(),
            description: "Switch to last viewed tag".to_string(),
            category: Category::Navigation,
        },

        // Layout Management
        Keybinding {
            modifiers: vec!["Super".to_string()],
            key: "h".to_string(),
            function: "Shrink Master".to_string(),
            description: "Decrease master area size".to_string(),
            category: Category::Layout,
        },
        Keybinding {
            modifiers: vec!["Super".to_string()],
            key: "l".to_string(),
            function: "Expand Master".to_string(),
            description: "Increase master area size".to_string(),
            category: Category::Layout,
        },
        Keybinding {
            modifiers: vec!["Super".to_string()],
            key: "i".to_string(),
            function: "Inc Master".to_string(),
            description: "Increase number of masters".to_string(),
            category: Category::Layout,
        },
        Keybinding {
            modifiers: vec!["Super".to_string()],
            key: "t".to_string(),
            function: "Dwindle Layout".to_string(),
            description: "Set layout to dwindle".to_string(),
            category: Category::Layout,
        },
        Keybinding {
            modifiers: vec!["Super".to_string(), "Shift".to_string()],
            key: "f".to_string(),
            function: "Grid Layout".to_string(),
            description: "Set layout to grid".to_string(),
            category: Category::Layout,
        },
        Keybinding {
            modifiers: vec!["Super".to_string()],
            key: "Space".to_string(),
            function: "Cycle Layout".to_string(),
            description: "Cycle through layouts".to_string(),
            category: Category::Layout,
        },

        // Gap Management
        Keybinding {
            modifiers: vec!["Super".to_string(), "Ctrl".to_string()],
            key: "t".to_string(),
            function: "Toggle Gaps".to_string(),
            description: "Toggle gaps on/off".to_string(),
            category: Category::Gaps,
        },
        Keybinding {
            modifiers: vec!["Super".to_string(), "Ctrl".to_string()],
            key: "i".to_string(),
            function: "Increase All Gaps".to_string(),
            description: "Increase all gaps".to_string(),
            category: Category::Gaps,
        },
        Keybinding {
            modifiers: vec!["Super".to_string(), "Ctrl".to_string()],
            key: "d".to_string(),
            function: "Decrease All Gaps".to_string(),
            description: "Decrease all gaps".to_string(),
            category: Category::Gaps,
        },
        Keybinding {
            modifiers: vec!["Super".to_string(), "Ctrl".to_string(), "Shift".to_string()],
            key: "d".to_string(),
            function: "Default Gaps".to_string(),
            description: "Reset gaps to default".to_string(),
            category: Category::Gaps,
        },

        // Tags (1-9)
        Keybinding {
            modifiers: vec!["Super".to_string()],
            key: "1-9".to_string(),
            function: "View Tag".to_string(),
            description: "Switch to tag 1-9".to_string(),
            category: Category::Tags,
        },
        Keybinding {
            modifiers: vec!["Super".to_string(), "Shift".to_string()],
            key: "1-9".to_string(),
            function: "Move to Tag".to_string(),
            description: "Move window to tag 1-9".to_string(),
            category: Category::Tags,
        },
        Keybinding {
            modifiers: vec!["Super".to_string(), "Ctrl".to_string()],
            key: "1-9".to_string(),
            function: "Toggle Tag View".to_string(),
            description: "Toggle view of tag 1-9".to_string(),
            category: Category::Tags,
        },

        // System
        Keybinding {
            modifiers: vec!["Super".to_string(), "Ctrl".to_string()],
            key: "q".to_string(),
            function: "Quit DWM".to_string(),
            description: "Kill dwm and bar".to_string(),
            category: Category::System,
        },
        Keybinding {
            modifiers: vec!["Super".to_string(), "Shift".to_string()],
            key: "r".to_string(),
            function: "Restart DWM".to_string(),
            description: "Restart dwm".to_string(),
            category: Category::System,
        },
        Keybinding {
            modifiers: vec!["Alt".to_string()],
            key: "l".to_string(),
            function: "Lock Screen".to_string(),
            description: "Lock the screen".to_string(),
            category: Category::System,
        },
        Keybinding {
            modifiers: vec!["Alt".to_string()],
            key: "x".to_string(),
            function: "Power Menu".to_string(),
            description: "Open power menu".to_string(),
            category: Category::System,
        },
        Keybinding {
            modifiers: vec!["Super".to_string()],
            key: "m".to_string(),
            function: "Keyboard Layout".to_string(),
            description: "Switch keyboard layout".to_string(),
            category: Category::System,
        },

        // Borders
        Keybinding {
            modifiers: vec!["Super".to_string(), "Shift".to_string()],
            key: "minus".to_string(),
            function: "Decrease Border".to_string(),
            description: "Decrease border width".to_string(),
            category: Category::Borders,
        },
        Keybinding {
            modifiers: vec!["Super".to_string(), "Shift".to_string()],
            key: "p".to_string(),
            function: "Increase Border".to_string(),
            description: "Increase border width".to_string(),
            category: Category::Borders,
        },
        Keybinding {
            modifiers: vec!["Super".to_string(), "Shift".to_string()],
            key: "w".to_string(),
            function: "Default Border".to_string(),
            description: "Reset border to default".to_string(),
            category: Category::Borders,
        },
    ]
}
