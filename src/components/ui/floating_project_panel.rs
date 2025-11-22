/// Floating project selector panel
use dioxus::prelude::*;
use uuid::Uuid;

/// Floating project panel component
#[component]
pub fn FloatingProjectPanel(
    visible: bool,
    selected_project_id: Option<Uuid>,
    is_none_selected: bool,
    search_query: String,
    on_search_change: EventHandler<String>,
    on_project_select: EventHandler<Uuid>,
    on_none_select: EventHandler<()>,
    on_close: EventHandler<()>,
) -> Element {
    if !visible {
        return rsx! {};
    }

    let plan = crate::state::use_plan_state();
    let plan_data = plan();

    let mut is_collapsed = use_signal(|| false);

    // Filter projects by search query
    let search_lower = search_query.to_lowercase();
    let filtered_projects: Vec<_> = plan_data
        .technical_projects
        .iter()
        .filter(|p| search_query.is_empty() || p.name.to_lowercase().contains(&search_lower))
        .collect();

    let panel_class = if is_collapsed() {
        "floating-project-panel collapsed"
    } else {
        "floating-project-panel"
    };

    rsx! {
        // Backdrop to capture clicks outside
        div {
            class: "floating-panel-backdrop",
            onclick: move |_| on_close.call(()),

            // Panel
            div {
                class: "{panel_class}",
                onclick: move |e| e.stop_propagation(),

                if is_collapsed() {
                    // Minimized state - just expand button
                    button {
                        class: "panel-expand-button",
                        onclick: move |_| is_collapsed.set(false),
                        "‹‹"
                    }
                } else {
                    // Expanded state - full panel
                    // Header with search and collapse button
                    div { class: "panel-header",
                        input {
                            class: "panel-search",
                            r#type: "text",
                            placeholder: "Search projects...",
                            value: "{search_query}",
                            oninput: move |e| on_search_change.call(e.value()),
                        }
                        button {
                            class: "panel-collapse-button",
                            onclick: move |_| is_collapsed.set(true),
                            title: "Minimize panel",
                            "››"
                        }
                    }

                // Project list
                div { class: "panel-project-list",
                    for project in filtered_projects {
                        {
                            let project_id = project.id;
                            let allocated = plan_data.calculate_project_allocated_weeks(&project_id);
                            let color = project.get_color(&plan_data);
                            let is_selected = selected_project_id == Some(project_id);

                            rsx! {
                                button {
                                    key: "{project_id}",
                                    class: if is_selected { "panel-project-option selected" } else { "panel-project-option" },
                                    onclick: move |_| on_project_select.call(project_id),
                                    div {
                                        class: "project-color-dot",
                                        style: "background: {color.to_hex()};",
                                    }
                                    span { class: "project-name", "{project.name}" }
                                    span { class: "project-allocated", "{allocated:.1}w" }
                                }
                            }
                        }
                    }

                    // Clear/None option
                    button {
                        class: if is_none_selected { "panel-project-option clear-option selected" } else { "panel-project-option clear-option" },
                        onclick: move |_| on_none_select.call(()),
                        span { class: "clear-indicator", "✕" }
                        span { class: "project-name", "Clear" }
                    }
                    }
                }
            }
        }
    }
}
