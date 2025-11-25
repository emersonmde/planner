use dioxus::prelude::*;
use planner_core::models::ProjectColor;

/// Position of a cell in a multi-week series
#[allow(dead_code)] // Reserved for future multi-week series detection
#[derive(Clone, Copy, PartialEq)]
pub enum CellPosition {
    /// Standalone cell (not part of a series)
    Standalone,
    /// First cell in a series (rounded left, square right)
    First,
    /// Middle cell in a series (square all sides)
    Middle,
    /// Last cell in a series (square left, rounded right)
    Last,
}

/// Grid cell variant determining visual style
#[derive(Clone, PartialEq)]
pub enum GridCellVariant {
    /// Empty unallocated cell with dashed border and + icon
    Empty,
    /// Single week project allocation
    SingleWeek {
        project_name: String,
        project_color: ProjectColor,
        percentage: f32,
        is_before_start: bool,
    },
    /// Multi-week project allocation (part of connected series)
    #[allow(dead_code)] // Reserved for future multi-week series detection
    MultiWeek {
        project_name: String,
        project_color: ProjectColor,
        percentage: f32,
        position: CellPosition,
        /// Total duration in weeks (shown only on last cell)
        duration_weeks: Option<usize>,
        is_before_start: bool,
    },
    /// Split allocation between two projects (vertical split)
    Split {
        project1_name: String,
        project1_color: ProjectColor,
        project1_percentage: f32,
        project2_name: String,
        project2_color: ProjectColor,
        project2_percentage: f32,
    },
}

/// Allocation grid cell component
/// Reference: docs/ui-design.md section 5.3
#[component]
pub fn GridCell(
    variant: GridCellVariant,
    #[props(default)] onclick: Option<EventHandler<MouseEvent>>,
) -> Element {
    match variant {
        GridCellVariant::Empty => {
            rsx! {
                div {
                    class: "grid-cell grid-cell-empty",
                    onclick: move |evt| {
                        if let Some(handler) = &onclick {
                            handler.call(evt);
                        }
                    },
                    div { class: "empty-icon", "+" }
                }
            }
        }
        GridCellVariant::SingleWeek {
            project_name,
            project_color,
            percentage,
            is_before_start,
        } => {
            let color_hex = project_color.to_hex();
            let cell_class = if is_before_start {
                "grid-cell grid-cell-allocated grid-cell-before-start"
            } else {
                "grid-cell grid-cell-allocated"
            };

            rsx! {
                div {
                    class: "{cell_class}",
                    style: "--project-color: {color_hex};",
                    onclick: move |evt| {
                        if let Some(handler) = &onclick {
                            handler.call(evt);
                        }
                    },
                    div { class: "cell-content",
                        span { class: "project-text", "{project_name}" }
                        if percentage < 100.0 {
                            span { class: "allocation-badge", "{percentage:.0}%" }
                        }
                    }
                    if is_before_start {
                        div { class: "before-start-overlay",
                            span { class: "warning-icon", "!" }
                        }
                    }
                }
            }
        }
        GridCellVariant::MultiWeek {
            project_name,
            project_color,
            percentage,
            position,
            duration_weeks,
            is_before_start,
        } => {
            let color_hex = project_color.to_hex();
            let position_class = match position {
                CellPosition::First => "grid-cell-multi-first",
                CellPosition::Middle => "grid-cell-multi-middle",
                CellPosition::Last => "grid-cell-multi-last",
                CellPosition::Standalone => "grid-cell-multi-standalone",
            };
            let base_class = if is_before_start {
                "grid-cell grid-cell-allocated grid-cell-multi grid-cell-before-start"
            } else {
                "grid-cell grid-cell-allocated grid-cell-multi"
            };
            let cell_class = format!("{} {}", base_class, position_class);

            rsx! {
                div {
                    class: "{cell_class}",
                    style: "--project-color: {color_hex};",
                    onclick: move |evt| {
                        if let Some(handler) = &onclick {
                            handler.call(evt);
                        }
                    },
                    div { class: "cell-content",
                        span { class: "project-text", "{project_name}" }
                        if let Some(weeks) = duration_weeks {
                            if position == CellPosition::Last {
                                span { class: "duration-badge", "{weeks}w" }
                            }
                        }
                        if percentage < 100.0 && position != CellPosition::Middle {
                            span { class: "allocation-badge", "{percentage:.0}%" }
                        }
                    }
                    if is_before_start {
                        div { class: "before-start-overlay",
                            span { class: "warning-icon", "!" }
                        }
                    }
                }
            }
        }
        GridCellVariant::Split {
            project1_name,
            project1_color,
            project1_percentage,
            project2_name,
            project2_color,
            project2_percentage,
        } => {
            let color1_hex = project1_color.to_hex();
            let color2_hex = project2_color.to_hex();

            rsx! {
                div {
                    class: "grid-cell grid-cell-split",
                    onclick: move |evt| {
                        if let Some(handler) = &onclick {
                            handler.call(evt);
                        }
                    },
                    div {
                        class: "split-section split-top",
                        style: "--project-color: {color1_hex};",
                        span { class: "split-text", "{project1_name}" }
                        span { class: "split-percentage", "{project1_percentage:.0}%" }
                    }
                    div {
                        class: "split-section",
                        style: "--project-color: {color2_hex};",
                        span { class: "split-text", "{project2_name}" }
                        span { class: "split-percentage", "{project2_percentage:.0}%" }
                    }
                }
            }
        }
    }
}
