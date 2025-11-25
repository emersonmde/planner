use dioxus::prelude::*;

/// Generic data table component with header and rows
#[component]
pub fn DataTable(children: Element) -> Element {
    rsx! {
        div { class: "data-table",
            {children}
        }
    }
}

/// Table header row
#[component]
pub fn TableHeader(children: Element) -> Element {
    rsx! {
        div { class: "table-header",
            {children}
        }
    }
}

/// Table header cell
#[component]
pub fn TableHeaderCell(children: Element) -> Element {
    rsx! {
        div { class: "table-header-cell",
            {children}
        }
    }
}

/// Table body row
#[component]
pub fn TableRow(children: Element) -> Element {
    rsx! {
        div { class: "table-row",
            {children}
        }
    }
}

/// Table cell with optional styling variants
#[derive(Clone, Copy, PartialEq, Default)]
pub enum CellStyle {
    #[default]
    Default,
    Emphasis,
    Secondary,
    #[allow(dead_code)] // Reserved for future monospace cell content
    Monospace,
    MonospaceSecondary,
    MonospaceEmphasis,
}

#[component]
pub fn TableCell(#[props(default)] style: CellStyle, children: Element) -> Element {
    let class_name = match style {
        CellStyle::Default => "table-cell",
        CellStyle::Emphasis => "table-cell emphasis",
        CellStyle::Secondary => "table-cell secondary",
        CellStyle::Monospace => "table-cell monospace",
        CellStyle::MonospaceSecondary => "table-cell monospace secondary",
        CellStyle::MonospaceEmphasis => "table-cell monospace emphasis",
    };

    rsx! {
        div { class: "{class_name}",
            {children}
        }
    }
}

/// Project name with colored dot
#[component]
pub fn ProjectName(name: String, color: String) -> Element {
    rsx! {
        div { class: "project-name",
            span {
                class: "project-dot",
                style: "background: {color}",
            }
            "{name}"
        }
    }
}
