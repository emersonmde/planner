use dioxus::prelude::*;

/// Keybindings help overlay showing all keyboard shortcuts
/// Reference: Phase 7 - Keybindings Help Overlay
#[component]
pub fn KeybindingsOverlay(visible: bool, on_close: EventHandler<()>) -> Element {
    if !visible {
        return rsx! {};
    }

    rsx! {
        div {
            class: "keybindings-overlay-backdrop",
            onclick: move |_| on_close.call(()),

            div {
                class: "keybindings-overlay",
                onclick: move |evt| evt.stop_propagation(),

                div { class: "keybindings-header",
                    h2 { "Keyboard Shortcuts" }
                    button {
                        class: "keybindings-close",
                        onclick: move |_| on_close.call(()),
                        "✕"
                    }
                }

                div { class: "keybindings-content",
                    div { class: "keybindings-section",
                        h3 { "General" }
                        div { class: "keybinding-row",
                            span { class: "keybinding-keys", "?" }
                            span { class: "keybinding-desc", "Toggle this help" }
                        }
                        div { class: "keybinding-row",
                            span { class: "keybinding-keys", "Esc" }
                            span { class: "keybinding-desc", "Close modals / Exit paintbrush mode" }
                        }
                    }

                    div { class: "keybindings-section",
                        h3 { "Allocation Editing" }
                        div { class: "keybinding-row",
                            span { class: "keybinding-keys", "Cmd/Ctrl + C" }
                            span { class: "keybinding-desc", "Copy hovered cell allocation" }
                        }
                        div { class: "keybinding-row",
                            span { class: "keybinding-keys", "Cmd/Ctrl + V" }
                            span { class: "keybinding-desc", "Paste to hovered cell" }
                        }
                        div { class: "keybinding-row",
                            span { class: "keybinding-keys", "Delete / Backspace" }
                            span { class: "keybinding-desc", "Clear hovered cell" }
                        }
                        div { class: "keybinding-row",
                            span { class: "keybinding-keys", "Right Click" }
                            span { class: "keybinding-desc", "Open context menu (Assign / Split / Clear)" }
                        }
                    }

                    div { class: "keybindings-section",
                        h3 { "Paintbrush Mode" }
                        div { class: "keybinding-row",
                            span { class: "keybinding-keys", "Click FAB" }
                            span { class: "keybinding-desc", "Open project panel" }
                        }
                        div { class: "keybinding-row",
                            span { class: "keybinding-keys", "Click" }
                            span { class: "keybinding-desc", "Allocate selected project to cell" }
                        }
                        div { class: "keybinding-row",
                            span { class: "keybinding-keys", "Drag" }
                            span { class: "keybinding-desc", "Paint across multiple cells" }
                        }
                        div { class: "keybinding-row",
                            span { class: "keybinding-keys", "Esc" }
                            span { class: "keybinding-desc", "Exit paintbrush mode" }
                        }
                    }
                }
            }
        }
    }
}
