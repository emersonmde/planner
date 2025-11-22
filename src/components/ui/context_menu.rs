/// Context menu component for right-click interactions
use dioxus::prelude::*;

/// Menu item action
#[derive(Clone, PartialEq, Debug)]
pub enum MenuAction {
    AssignProject,
    SplitAllocation,
    EditSplit,
    ClearAssignment,
}

/// Context menu component
#[component]
pub fn ContextMenu(
    x: i32,
    y: i32,
    visible: bool,
    has_allocation: bool,
    is_split: bool,
    on_action: EventHandler<MenuAction>,
    on_close: EventHandler<()>,
) -> Element {
    if !visible {
        return rsx! {};
    }

    rsx! {
        // Backdrop to capture clicks outside
        div {
            class: "context-menu-backdrop",
            onclick: move |_| on_close.call(()),

            // Menu positioned at cursor
            div {
                class: "context-menu",
                style: "left: {x}px; top: {y}px;",
                onclick: move |e| e.stop_propagation(), // Prevent backdrop click

                // Assign Project
                button {
                    class: "context-menu-item",
                    onclick: move |_| {
                        on_action.call(MenuAction::AssignProject);
                        on_close.call(());
                    },
                    span { class: "menu-icon", "📌" }
                    span { class: "menu-label", "Assign Project..." }
                }

                // Split Allocation or Edit Split
                if is_split {
                    button {
                        class: "context-menu-item",
                        onclick: move |_| {
                            on_action.call(MenuAction::EditSplit);
                            on_close.call(());
                        },
                        span { class: "menu-icon", "✏️" }
                        span { class: "menu-label", "Edit Split Allocation..." }
                    }
                } else {
                    button {
                        class: "context-menu-item",
                        onclick: move |_| {
                            on_action.call(MenuAction::SplitAllocation);
                            on_close.call(());
                        },
                        span { class: "menu-icon", "⚡" }
                        span { class: "menu-label", "Split Allocation..." }
                    }
                }

                // Separator
                if has_allocation {
                    div { class: "context-menu-separator" }
                }

                // Clear Assignment
                if has_allocation {
                    button {
                        class: "context-menu-item",
                        onclick: move |_| {
                            on_action.call(MenuAction::ClearAssignment);
                            on_close.call(());
                        },
                        span { class: "menu-icon", "🗑" }
                        span { class: "menu-label", "Clear Assignment" }
                    }
                }
            }
        }
    }
}
