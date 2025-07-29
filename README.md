# DWM Keybinding Cheatsheet

A GTK4-based application that provides a visual reference for your DWM (Dynamic Window Manager) keybindings. This application displays all your DWM shortcuts in an organized, searchable interface with a modern dark theme inspired by Catppuccin colors.

## Features

- **Categorized Display**: Keybindings organized by function (Media, Navigation, Window Management, etc.)
- **Dark Theme**: Modern Catppuccin-inspired color scheme with external CSS files
- **Real-time Search**: Filter keybindings as you type
- **Keyboard Shortcuts**: Press Escape to close the application
- **Clean Interface**: Easy-to-read layout with proper spacing and typography
- **Customizable Styling**: External CSS files for easy theme modification
- **Category Filtering**: Browse keybindings by category using the sidebar

## Keybinding Categories

The application organizes your DWM keybindings into the following categories:

- **Media Controls**: Volume, brightness, and media playback
- **Screenshots**: Screen capture functionality
- **Applications**: Launch programs and tools
- **Window Management**: Control window behavior
- **Navigation**: Focus and move between windows
- **Layout Management**: Change and control layouts
- **Gap Management**: Adjust window gaps
- **Tags & Workspaces**: Switch between workspaces
- **System**: System controls and DWM management
- **Borders**: Window border customization

## Installation

### Prerequisites

- Rust (latest stable version)
- GTK4 development libraries
- GCC or Clang compiler

### On Arch Linux

```bash
sudo pacman -S rust gtk4 base-devel
```

### On Ubuntu/Debian

```bash
sudo apt install rustc libgtk-4-dev build-essential
```

### Build from Source

1. Clone the repository:
   ```bash
   git clone <repository-url>
   cd dwm-cheatsheet
   ```

2. Build the application:
   ```bash
   cargo build --release
   ```

3. Run the application:
   ```bash
   cargo run --release
   ```

## Usage

1. Launch the application using `cargo run` or by running the compiled binary
2. Browse different categories using the sidebar on the left
3. Use the search bar at the top to find specific keybindings
4. Each keybinding shows:
   - The key combination (e.g., Super + Return)
   - The function name (e.g., "Terminal")
   - A description of what it does

## Keyboard Shortcuts

- **Escape**: Close the application
- **Type in search bar**: Filter keybindings in real-time

## CSS Styling and Themes

The application uses external CSS files for easy theming and customization:

### CSS Structure

```
styles/
├── main.css           # Currently active theme (rename desired theme to this)
├── catppuccin.css     # Catppuccin Mocha theme
├── gruvbox.css        # Gruvbox Material theme
├── nord.css           # Nord theme
├── dracula.css        # Dracula theme
├── onedark.css        # OneDark theme
└── solarized.css      # Solarized Dark theme

```

### Theme Switching

To switch between themes, rename the desired theme file to `main.css`:

1. **Backup current theme** (optional):
   ```bash
   mv styles/main.css styles/main.css.backup
   ```

2. **Choose your desired theme** and rename it:
   ```bash
   # For Catppuccin theme
   cp styles/catppuccin.css styles/main.css
   
   # For Gruvbox theme
   cp styles/gruvbox.css styles/main.css
   
   # For Nord theme
   cp styles/nord.css styles/main.css
   
   # For Dracula theme
   cp styles/dracula.css styles/main.css
   
   # For OneDark theme
   cp styles/onedark.css styles/main.css
   
   # For Solarized theme
   cp styles/solarized.css styles/main.css
   
   # For Caelestia theme
   cp styles/caelestia.css styles/main.css
   ```

3. **Restart the application** to see the new theme

### Available Themes

- **Catppuccin**: Modern purple-tinted dark theme with excellent readability
- **Gruvbox**: Warm, retro-inspired theme with earthy tones
- **Nord**: Cool, arctic-inspired theme with blue and gray tones
- **Dracula**: High-contrast theme with purple accents
- **OneDark**: Clean, professional dark theme inspired by Atom's One Dark
- **Solarized**: Scientifically designed theme optimized for readability
- **Caelestia**: Cosmic-inspired theme with deep space aesthetics

### Customizing Themes

Each theme file is a complete, standalone CSS file with all styling rules. To create a custom theme:

- change the theme simply by renaming the .css file to main.css

1. **Copy an existing theme** as a starting point:
   ```bash
   cp styles/catppuccin.css styles/mytheme.css
   ```

2. **Edit the colors** in your new theme file:
   ```css
   /* In styles/mytheme.css */
   .main-container {
       background-color: #your-bg-color;
       color: #your-text-color;
       /* ... */
   }
All themes follow the same CSS structure, ensuring consistency while allowing complete color customization.

## Customization

The keybindings are defined in `src/keybindings.rs`. To customize them for your DWM setup:

1. Edit the `get_keybindings()` function
2. Add, remove, or modify keybindings as needed
3. Rebuild the application

Example keybinding structure:
```rust
Keybinding {
    modifiers: vec!["Super".to_string()],
    key: "Return".to_string(),
    function: "Terminal".to_string(),
    description: "Launch Kitty terminal".to_string(),
    category: Category::Applications,
},
```

## Configuration

The application includes keybindings based on a typical DWM configuration with:
- Super (Windows) key as the main modifier
- Various applications (Firefox, VS Code, Kitty terminal, etc.)
- Gap management patches
- Multiple layout support
- Media controls

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Test thoroughly
5. Submit a pull request

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Screenshots

The application features:
- A clean, modern interface with a dark theme
- Color-coded categories for easy navigation
- Keyboard shortcuts displayed as styled key combinations
- Responsive layout that works on different screen sizes
Writting a cheat sheet in rust for my dwm config
