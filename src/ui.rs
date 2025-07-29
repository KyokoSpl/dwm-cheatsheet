use crate::keybindings::{get_keybindings, Category, Keybinding};
use gtk4::prelude::*;
use gtk4::{
    Align, Box, Label, ListBox, Orientation, Paned, ScrolledWindow,
    SearchEntry, Separator, Widget,
};
use std::collections::HashMap;

pub fn build_main_widget() -> Widget {
    let main_box = Box::builder()
        .orientation(Orientation::Vertical)
        .spacing(0)
        .build();
    main_box.add_css_class("main-container");
    main_box.set_hexpand(true);
    main_box.set_vexpand(true);

    // Header
    let header = build_header();
    main_box.append(&header);

    // Create content area first so we can reference it in search
    let content_area = build_content_area();
    
    // Search bar with functionality
    let search_entry = SearchEntry::builder()
        .placeholder_text("Search keybindings...")
        .margin_start(20)
        .margin_end(20)
        .margin_top(10)
        .margin_bottom(10)
        .build();

    // Connect search functionality
    let content_area_clone = content_area.clone();
    search_entry.connect_search_changed(move |entry| {
        let search_text = entry.text().to_string().to_lowercase();
        filter_content(&content_area_clone, &search_text);
    });

    main_box.append(&search_entry);

    // Content area with sidebar and main content
    let paned = Paned::new(Orientation::Horizontal);
    paned.set_shrink_start_child(false);
    paned.set_shrink_end_child(true);
    paned.set_position(300);
    paned.set_resize_start_child(false);
    paned.set_resize_end_child(true);
    paned.set_hexpand(true);
    paned.set_vexpand(true);
    paned.set_wide_handle(true);

    // Sidebar with categories
    let sidebar = build_sidebar();
    paned.set_start_child(Some(&sidebar));
    
    println!("Sidebar built and set as start child");

    paned.set_end_child(Some(&content_area));
    
    println!("Content area built and set as end child");

    main_box.append(&paned);

    // Add CSS styling
    add_css_styling();

    main_box.upcast()
}

fn build_header() -> Widget {
    let header_box = Box::builder()
        .orientation(Orientation::Vertical)
        .spacing(5)
        .build();
    header_box.add_css_class("header");
    header_box.set_margin_start(20);
    header_box.set_margin_end(20);
    header_box.set_margin_top(20);
    header_box.set_margin_bottom(10);

    let title = Label::new(Some("DWM Keybinding Cheatsheet"));
    title.add_css_class("title");
    title.set_halign(Align::Center);

    let subtitle = Label::new(Some("Quick reference for your DWM configuration"));
    subtitle.add_css_class("subtitle");
    subtitle.set_halign(Align::Center);

    header_box.append(&title);
    header_box.append(&subtitle);

    let separator = Separator::new(Orientation::Horizontal);
    separator.set_margin_top(10);
    header_box.append(&separator);

    header_box.upcast()
}

fn build_sidebar() -> Widget {
    let sidebar_box = Box::builder()
        .orientation(Orientation::Vertical)
        .spacing(0)
        .build();
    sidebar_box.add_css_class("sidebar");
    sidebar_box.set_width_request(300);
    sidebar_box.set_hexpand(false);
    sidebar_box.set_vexpand(true);

    let sidebar_title = Label::new(Some("Categories"));
    sidebar_title.add_css_class("sidebar-title");
    sidebar_title.set_margin_start(15);
    sidebar_title.set_margin_end(15);
    sidebar_title.set_margin_top(15);
    sidebar_title.set_margin_bottom(10);
    sidebar_title.set_halign(Align::Start);

    sidebar_box.append(&sidebar_title);

    let scrolled = ScrolledWindow::builder()
        .hscrollbar_policy(gtk4::PolicyType::Never)
        .vscrollbar_policy(gtk4::PolicyType::Automatic)
        .hexpand(false)
        .vexpand(true)
        .build();

    let listbox = ListBox::new();
    listbox.add_css_class("category-list");

    // Add "All" category
    let all_row = create_category_row("All Categories", "#cdd6f4", true);
    listbox.append(&all_row);

    // Add separator
    let separator = Separator::new(Orientation::Horizontal);
    separator.set_margin_top(5);
    separator.set_margin_bottom(5);
    listbox.append(&separator);

    // Add categories
    let categories = [
        Category::Media,
        Category::Screenshot,
        Category::Applications,
        Category::WindowManagement,
        Category::Layout,
        Category::Gaps,
        Category::Navigation,
        Category::Tags,
        Category::System,
        Category::Borders,
    ];

    for category in categories {
        let row = create_category_row(category.as_str(), category.color(), false);
        listbox.append(&row);
    }

    scrolled.set_child(Some(&listbox));
    sidebar_box.append(&scrolled);

    sidebar_box.upcast()
}

fn create_category_row(name: &str, _color: &str, is_selected: bool) -> Widget {
    let row_box = Box::builder()
        .orientation(Orientation::Horizontal)
        .spacing(10)
        .build();
    row_box.set_margin_start(15);
    row_box.set_margin_end(15);
    row_box.set_margin_top(8);
    row_box.set_margin_bottom(8);

    if is_selected {
        row_box.add_css_class("category-row-selected");
    } else {
        row_box.add_css_class("category-row");
    }

    // Color indicator
    let color_box = Box::builder()
        .orientation(Orientation::Horizontal)
        .spacing(0)
        .build();
    color_box.set_width_request(4);
    color_box.set_height_request(20);
    color_box.add_css_class("color-indicator");
    
    // Category name
    let label = Label::new(Some(name));
    label.set_halign(Align::Start);
    label.add_css_class("category-label");

    row_box.append(&color_box);
    row_box.append(&label);

    row_box.upcast()
}

fn build_content_area() -> Widget {
    let content_box = Box::builder()
        .orientation(Orientation::Vertical)
        .spacing(0)
        .build();
    content_box.add_css_class("content-area");
    content_box.set_hexpand(true);
    content_box.set_vexpand(true);

    let scrolled = ScrolledWindow::builder()
        .hscrollbar_policy(gtk4::PolicyType::Never)
        .vscrollbar_policy(gtk4::PolicyType::Automatic)
        .hexpand(true)
        .vexpand(true)
        .build();

    let keybindings_box = Box::builder()
        .orientation(Orientation::Vertical)
        .spacing(15)
        .build();
    keybindings_box.set_margin_start(20);
    keybindings_box.set_margin_end(20);
    keybindings_box.set_margin_top(20);
    keybindings_box.set_margin_bottom(20);
    keybindings_box.set_vexpand(true);

    // Group keybindings by category
    let keybindings = get_keybindings();
    println!("Total keybindings loaded: {}", keybindings.len());
    let mut grouped_keybindings: HashMap<Category, Vec<Keybinding>> = HashMap::new();

    for keybinding in keybindings {
        grouped_keybindings
            .entry(keybinding.category.clone())
            .or_default()
            .push(keybinding);
    }

    // Create sections for each category
    let categories = [
        Category::Media,
        Category::Screenshot,
        Category::Applications,
        Category::WindowManagement,
        Category::Navigation,
        Category::Layout,
        Category::Gaps,
        Category::Tags,
        Category::System,
        Category::Borders,
    ];

    for category in categories {
        if let Some(category_keybindings) = grouped_keybindings.get(&category) {
            println!("Adding category: {:?} with {} keybindings", category, category_keybindings.len());
            let section = create_category_section(&category, category_keybindings);
            keybindings_box.append(&section);
        }
    }

    scrolled.set_child(Some(&keybindings_box));
    content_box.append(&scrolled);
    

    content_box.upcast()
}

fn create_category_section(category: &Category, keybindings: &[Keybinding]) -> Widget {
    let section_box = Box::builder()
        .orientation(Orientation::Vertical)
        .spacing(10)
        .build();
    section_box.add_css_class("category-section");
    section_box.set_margin_bottom(20);
    
    println!("Creating section for category: {:?} with {} keybindings", category, keybindings.len());

    // Category header
    let header_box = Box::builder()
        .orientation(Orientation::Horizontal)
        .spacing(10)
        .build();
    header_box.add_css_class("category-header");

    let color_indicator = Box::builder()
        .orientation(Orientation::Horizontal)
        .spacing(0)
        .build();
    color_indicator.set_width_request(4);
    color_indicator.set_height_request(24);
    color_indicator.add_css_class("category-color");

    let title = Label::new(Some(category.as_str()));
    title.add_css_class("category-title");
    title.set_halign(Align::Start);

    header_box.append(&color_indicator);
    header_box.append(&title);

    section_box.append(&header_box);

    // Keybindings grid
    let keybindings_box = Box::builder()
        .orientation(Orientation::Vertical)
        .spacing(5)
        .build();
    keybindings_box.add_css_class("keybindings-container");

    for keybinding in keybindings {
        let keybinding_row = create_keybinding_row(keybinding);
        keybindings_box.append(&keybinding_row);
        println!("Added keybinding: {}", keybinding.function);
    }

    section_box.append(&keybindings_box);

    section_box.upcast()
}

fn create_keybinding_row(keybinding: &Keybinding) -> Widget {
    let row_box = Box::builder()
        .orientation(Orientation::Horizontal)
        .spacing(15)
        .build();
    row_box.add_css_class("keybinding-row");
    row_box.set_margin_start(10);
    row_box.set_margin_end(10);
    row_box.set_margin_top(8);
    row_box.set_margin_bottom(8);

    // Add search data as widget name for filtering
    let search_data = format!("{} {} {} {}", 
        keybinding.function.to_lowercase(),
        keybinding.description.to_lowercase(),
        keybinding.modifiers.join(" ").to_lowercase(),
        keybinding.key.to_lowercase()
    );
    row_box.set_widget_name(&search_data);

    // Keybinding display
    let key_box = Box::builder()
        .orientation(Orientation::Horizontal)
        .spacing(5)
        .build();
    key_box.add_css_class("key-combination");
    key_box.set_halign(Align::Start);
    key_box.set_width_request(350);

    // Add modifier keys
    for modifier in &keybinding.modifiers {
        let key_label = Label::new(Some(modifier));
        key_label.add_css_class("key");
        key_box.append(&key_label);

        if modifier != keybinding.modifiers.last().unwrap() || !keybinding.key.is_empty() {
            let plus = Label::new(Some("+"));
            plus.add_css_class("key-separator");
            key_box.append(&plus);
        }
    }

    // Add main key
    if !keybinding.key.is_empty() {
        let main_key = Label::new(Some(&keybinding.key));
        main_key.add_css_class("key");
        key_box.append(&main_key);
    }

    // Function name
    let function_label = Label::new(Some(&keybinding.function));
    function_label.add_css_class("function-name");
    function_label.set_halign(Align::Start);
    function_label.set_width_request(250);

    // Description
    let description_label = Label::new(Some(&keybinding.description));
    description_label.add_css_class("description");
    description_label.set_halign(Align::Start);
    description_label.set_ellipsize(gtk4::pango::EllipsizeMode::End);
    description_label.set_hexpand(true);

    row_box.append(&key_box);
    row_box.append(&function_label);
    row_box.append(&description_label);

    row_box.upcast()
}

fn add_css_styling() {
    let provider = gtk4::CssProvider::new();
    
    // Try to load from external CSS file first (from styles directory)
    let css_paths = ["styles/main.css", "styles.css"];
    let mut loaded = false;
    
    for path in css_paths {
        if let Ok(css_file) = std::fs::read_to_string(path) {
            provider.load_from_data(&css_file);
            println!("Loaded CSS from external file: {}", path);
            loaded = true;
            break;
        }
    }
    
    if !loaded {
        // Fallback to embedded CSS if no external file found
        println!("External CSS file not found, using embedded CSS");
        provider.load_from_data(include_str!("../styles/main.css"));
    }

    gtk4::style_context_add_provider_for_display(
        &gtk4::gdk::Display::default().expect("Could not connect to a display."),
        &provider,
        gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}

fn filter_content(content_area: &Widget, search_text: &str) {
    println!("Filtering with search text: '{}'", search_text);
    
    // Get the scrolled window from content area
    if let Some(content_box) = content_area.first_child() {
        if let Some(scrolled) = content_box.first_child() {
            if let Some(keybindings_box) = scrolled.first_child() {
                // Iterate through all category sections
                let mut child = keybindings_box.first_child();
                while let Some(section) = child {
                    let next_child = section.next_sibling();
                    
                    // Check if this is a category section
                    if section.css_classes().contains(&"category-section".into()) {
                        filter_category_section(&section, search_text);
                    }
                    
                    child = next_child;
                }
            }
        }
    }
}

fn filter_category_section(section: &Widget, search_text: &str) {
    let mut visible_keybindings = 0;
    
    // Find the keybindings container within the section
    let mut child = section.first_child();
    while let Some(current_child) = child {
        if current_child.css_classes().contains(&"keybindings-container".into()) {
            // Iterate through keybinding rows
            let mut row = current_child.first_child();
            while let Some(keybinding_row) = row {
                let next_row = keybinding_row.next_sibling();
                
                if keybinding_row.css_classes().contains(&"keybinding-row".into()) {
                    let search_data = keybinding_row.widget_name().to_lowercase();
                    let should_show = search_text.is_empty() || search_data.contains(search_text);
                    
                    keybinding_row.set_visible(should_show);
                    if should_show {
                        visible_keybindings += 1;
                    }
                }
                
                row = next_row;
            }
            break;
        }
        child = current_child.next_sibling();
    }
    
    // Hide the entire section if no keybindings are visible
    section.set_visible(visible_keybindings > 0);
}
