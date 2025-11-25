/// Settings modal for app-level configuration and tools
use chrono::NaiveDate;
use dioxus::prelude::*;

use crate::components::ui::{Button, ButtonVariant, ConfirmationDialog, Input};
use crate::state::{use_plan_state, use_preferences};

/// Props for SettingsModal
#[derive(Props, Clone, PartialEq)]
pub struct SettingsModalProps {
    /// Event handler for clearing preferences
    pub on_clear_preferences: EventHandler<()>,
    /// Event handler for loading sample data
    pub on_load_sample_data: EventHandler<()>,
    /// Event handler for closing the modal
    pub on_close: EventHandler<()>,
}

/// Settings modal component
#[component]
pub fn SettingsModal(props: SettingsModalProps) -> Element {
    // Access state
    let mut plan_state = use_plan_state();
    let mut preferences = use_preferences();
    let plan_data = plan_state();
    let prefs_data = preferences();

    // Confirmation dialog state
    let mut show_clear_confirmation = use_signal(|| false);
    let mut show_sample_data_confirmation = use_signal(|| false);

    // Plan configuration form state
    let plan_name = use_signal(|| plan_data.quarter_name.clone());
    let mut plan_start = use_signal(|| plan_data.quarter_start_date.format("%Y-%m-%d").to_string());
    let mut num_weeks = use_signal(|| plan_data.num_weeks.to_string());

    // Sprint configuration form state
    let mut sprint_anchor =
        use_signal(|| prefs_data.sprint_anchor_date.format("%Y-%m-%d").to_string());
    let mut sprint_length = use_signal(|| prefs_data.sprint_length_weeks.to_string());

    // Validation state
    let mut plan_error = use_signal(|| None::<String>);
    let mut sprint_error = use_signal(|| None::<String>);

    // Handle applying all settings
    let handle_apply = move |_| {
        let mut has_errors = false;

        // Validate and apply plan config
        let parsed_date = NaiveDate::parse_from_str(&plan_start(), "%Y-%m-%d");
        let parsed_weeks = num_weeks().parse::<usize>();

        match (parsed_date, parsed_weeks) {
            (Ok(date), Ok(weeks)) if (1..=52).contains(&weeks) => {
                plan_state.with_mut(|p| {
                    p.quarter_name = plan_name();
                    p.quarter_start_date = date;
                    p.num_weeks = weeks;
                });
                plan_error.set(None);
            }
            (Err(_), _) => {
                plan_error.set(Some("Invalid date format. Use YYYY-MM-DD.".to_string()));
                has_errors = true;
            }
            (_, Err(_)) => {
                plan_error.set(Some("Number of weeks must be a valid number.".to_string()));
                has_errors = true;
            }
            (_, Ok(weeks)) if !(1..=52).contains(&weeks) => {
                plan_error.set(Some(
                    "Number of weeks must be between 1 and 52.".to_string(),
                ));
                has_errors = true;
            }
            _ => {}
        }

        // Validate and apply sprint config
        let parsed_anchor = NaiveDate::parse_from_str(&sprint_anchor(), "%Y-%m-%d");
        let parsed_length = sprint_length().parse::<usize>();

        match (parsed_anchor, parsed_length) {
            (Ok(anchor), Ok(length)) if (1..=4).contains(&length) => {
                preferences.with_mut(|p| {
                    p.sprint_anchor_date = anchor;
                    p.sprint_length_weeks = length;
                });
                sprint_error.set(None);
            }
            (Err(_), _) => {
                sprint_error.set(Some("Invalid date format. Use YYYY-MM-DD.".to_string()));
                has_errors = true;
            }
            (_, Err(_)) => {
                sprint_error.set(Some("Sprint length must be a valid number.".to_string()));
                has_errors = true;
            }
            (_, Ok(length)) if !(1..=4).contains(&length) => {
                sprint_error.set(Some(
                    "Sprint length must be between 1 and 4 weeks.".to_string(),
                ));
                has_errors = true;
            }
            _ => {}
        }

        // Close modal if no errors
        if !has_errors {
            props.on_close.call(());
        }
    };

    rsx! {
        // Modal backdrop
        div {
            class: "modal-backdrop",
            onclick: move |_| props.on_close.call(()),

            // Modal container
            div {
                class: "modal-container settings-modal",
                onclick: move |e| e.stop_propagation(),

                // Header
                div { class: "modal-header",
                    h2 { class: "modal-title", "Settings" }
                    button {
                        class: "modal-close",
                        onclick: move |_| props.on_close.call(()),
                        "×"
                    }
                }

                // Body
                div { class: "modal-body",
                    // Plan Configuration section
                    div { class: "settings-section",
                        h3 { class: "settings-section-title", "Plan Configuration" }
                        p { class: "settings-section-description",
                            "Configure the planning period for this plan."
                        }

                        div { class: "settings-form",
                            div { class: "form-group",
                                label { class: "form-label", "Plan Name" }
                                Input {
                                    value: plan_name,
                                    placeholder: "e.g., Q1 2025, H1 2025".to_string(),
                                }
                            }

                            div { class: "form-row",
                                div { class: "form-group",
                                    label { class: "form-label", "Start Date" }
                                    input {
                                        r#type: "date",
                                        class: "form-input",
                                        value: "{plan_start()}",
                                        oninput: move |e: FormEvent| plan_start.set(e.value()),
                                    }
                                }

                                div { class: "form-group",
                                    label { class: "form-label", "Weeks" }
                                    input {
                                        r#type: "number",
                                        class: "form-input",
                                        value: "{num_weeks()}",
                                        min: "1",
                                        max: "52",
                                        oninput: move |e: FormEvent| num_weeks.set(e.value()),
                                    }
                                }
                            }

                            if let Some(error) = plan_error() {
                                div { class: "form-error", "{error}" }
                            }
                        }
                    }

                    // Sprint Configuration section
                    div { class: "settings-section",
                        h3 { class: "settings-section-title", "Sprint Configuration" }
                        p { class: "settings-section-description",
                            "Configure how sprints are calculated. The anchor date is a known sprint start; all other sprint boundaries are calculated relative to it."
                        }

                        div { class: "settings-form",
                            div { class: "form-row",
                                div { class: "form-group",
                                    label { class: "form-label", "Sprint Anchor Date" }
                                    input {
                                        r#type: "date",
                                        class: "form-input",
                                        value: "{sprint_anchor()}",
                                        oninput: move |e: FormEvent| sprint_anchor.set(e.value()),
                                    }
                                }

                                div { class: "form-group",
                                    label { class: "form-label", "Sprint Length (weeks)" }
                                    input {
                                        r#type: "number",
                                        class: "form-input",
                                        value: "{sprint_length()}",
                                        min: "1",
                                        max: "4",
                                        oninput: move |e: FormEvent| sprint_length.set(e.value()),
                                    }
                                }
                            }

                            if let Some(error) = sprint_error() {
                                div { class: "form-error", "{error}" }
                            }
                        }
                    }

                    // Storage section
                    div { class: "settings-section",
                        h3 { class: "settings-section-title", "Storage" }
                        p { class: "settings-section-description",
                            "Manage browser storage used by the application."
                        }

                        div { class: "settings-item",
                            div { class: "settings-item-info",
                                span { class: "settings-item-label", "Load Sample Data" }
                                span { class: "settings-item-description",
                                    "Load sample team members, projects, and allocations for demo/testing purposes."
                                }
                            }
                            Button {
                                variant: ButtonVariant::Secondary,
                                onclick: move |_| show_sample_data_confirmation.set(true),
                                "Load"
                            }
                        }

                        div { class: "settings-item",
                            div { class: "settings-item-info",
                                span { class: "settings-item-label", "Clear Preferences" }
                                span { class: "settings-item-description",
                                    "Reset team members, sprint config, and other preferences to defaults. Plan data (projects, allocations) will not be affected."
                                }
                            }
                            Button {
                                variant: ButtonVariant::Danger,
                                onclick: move |_| show_clear_confirmation.set(true),
                                "Clear"
                            }
                        }
                    }

                    // About section
                    div { class: "settings-section",
                        h3 { class: "settings-section-title", "About" }
                        div { class: "settings-about",
                            p { "Quarterly Planner v0.1.0" }
                            p { class: "settings-about-secondary",
                                "A Dioxus 0.7 application for engineering managers to plan quarterly resource allocation."
                            }
                        }
                    }
                }

                // Footer
                div { class: "modal-footer",
                    Button {
                        variant: ButtonVariant::Secondary,
                        onclick: move |_| props.on_close.call(()),
                        "Cancel"
                    }
                    Button {
                        variant: ButtonVariant::Primary,
                        onclick: handle_apply,
                        "Apply"
                    }
                }
            }

            // Clear Preferences Confirmation Dialog
            if show_clear_confirmation() {
                ConfirmationDialog {
                    visible: true,
                    title: "Clear Preferences".to_string(),
                    message: "Are you sure you want to clear all preferences?".to_string(),
                    warning: "This will reset team members, sprint configuration, and default capacity to their initial values. This action cannot be undone.".to_string(),
                    confirm_label: "Clear Preferences".to_string(),
                    on_confirm: move |_| {
                        props.on_clear_preferences.call(());
                        show_clear_confirmation.set(false);
                        props.on_close.call(());
                    },
                    on_cancel: move |_| {
                        show_clear_confirmation.set(false);
                    },
                }
            }

            // Load Sample Data Confirmation Dialog
            if show_sample_data_confirmation() {
                ConfirmationDialog {
                    visible: true,
                    title: "Load Sample Data".to_string(),
                    message: "Are you sure you want to load sample data?".to_string(),
                    warning: "This will replace your current team members, projects, and allocations with sample data. This action cannot be undone.".to_string(),
                    confirm_label: "Load Sample Data".to_string(),
                    on_confirm: move |_| {
                        props.on_load_sample_data.call(());
                        show_sample_data_confirmation.set(false);
                        props.on_close.call(());
                    },
                    on_cancel: move |_| {
                        show_sample_data_confirmation.set(false);
                    },
                }
            }
        }
    }
}
