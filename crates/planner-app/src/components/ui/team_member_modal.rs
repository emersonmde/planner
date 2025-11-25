use dioxus::prelude::*;
use uuid::Uuid;

use crate::components::ui::{Button, ButtonVariant, Input};
use planner_core::models::{Role, TeamMember};

/// Mode for the team member modal (Add or Edit)
#[derive(Clone, Copy, PartialEq)]
pub enum TeamMemberModalMode {
    Add,
    Edit(Uuid), // Contains team member ID being edited
}

/// Props for TeamMemberModal
#[derive(Props, Clone, PartialEq)]
pub struct TeamMemberModalProps {
    /// Mode (Add or Edit with team member ID)
    pub mode: TeamMemberModalMode,
    /// Initial values for the form
    pub initial_name: String,
    pub initial_role: Role,
    pub initial_capacity: f32,
    /// Default capacity for new team members (from preferences)
    pub default_capacity: f32,
    /// Current allocated weeks (for warning when reducing capacity below allocated)
    pub allocated_weeks: f32,
    /// Event handlers
    pub on_save: EventHandler<TeamMember>,
    pub on_cancel: EventHandler<()>,
}

/// Team member modal for adding/editing team members
#[component]
pub fn TeamMemberModal(props: TeamMemberModalProps) -> Element {
    // Form state - signals initialized from props
    let name = use_signal(|| props.initial_name.clone());
    let mut role = use_signal(|| props.initial_role);
    let mut capacity = use_signal(|| {
        if matches!(props.mode, TeamMemberModalMode::Add) && props.initial_capacity == 0.0 {
            props.default_capacity
        } else {
            props.initial_capacity
        }
    });

    // Validation errors and warnings
    let mut name_error = use_signal(String::new);
    let mut capacity_error = use_signal(String::new);
    let capacity_warning = use_memo(move || {
        if capacity() < props.allocated_weeks && props.allocated_weeks > 0.0 {
            format!(
                "Warning: {} already has {:.1} weeks allocated. Reducing capacity will show over-allocation.",
                if props.initial_name.is_empty() { "This member" } else { &props.initial_name },
                props.allocated_weeks
            )
        } else {
            String::new()
        }
    });

    // Validation function
    let mut validate_form = move || -> bool {
        let mut is_valid = true;

        // Validate name
        if name().trim().is_empty() {
            name_error.set("Name is required".to_string());
            is_valid = false;
        } else {
            name_error.set(String::new());
        }

        // Validate capacity
        if capacity() <= 0.0 {
            capacity_error.set("Capacity must be greater than 0".to_string());
            is_valid = false;
        } else {
            capacity_error.set(String::new());
        }

        is_valid
    };

    // Handle save
    let handle_save = move |_| {
        if !validate_form() {
            return;
        }

        let member = match props.mode {
            TeamMemberModalMode::Add => TeamMember {
                id: Uuid::new_v4(),
                name: name().trim().to_string(),
                role: role(),
                capacity: capacity(),
            },
            TeamMemberModalMode::Edit(id) => TeamMember {
                id,
                name: name().trim().to_string(),
                role: role(),
                capacity: capacity(),
            },
        };

        props.on_save.call(member);
    };

    let modal_title = match props.mode {
        TeamMemberModalMode::Add => "Add Team Member",
        TeamMemberModalMode::Edit(_) => "Edit Team Member",
    };

    rsx! {
        // Modal backdrop
        div {
            class: "modal-backdrop",
            onclick: move |_| props.on_cancel.call(()),

            // Modal container
            div {
                class: "modal-container team-member-modal",
                onclick: move |e| e.stop_propagation(),

                // Header
                div { class: "modal-header",
                    h2 { class: "modal-title", "{modal_title}" }
                    button {
                        class: "modal-close",
                        onclick: move |_| props.on_cancel.call(()),
                        "×"
                    }
                }

                // Body
                div { class: "modal-body",
                    // Name field
                    div { class: "form-field",
                        label { class: "form-label", "Name *" }
                        Input {
                            value: name,
                            placeholder: "e.g., Alice Smith".to_string(),
                        }
                        div { class: "form-error", "{name_error()}" }
                    }

                    // Role selection (radio buttons)
                    div { class: "form-field",
                        label { class: "form-label", "Role *" }
                        div { class: "role-selector",
                            label {
                                class: if role() == Role::Engineering { "role-option selected" } else { "role-option" },
                                input {
                                    r#type: "radio",
                                    name: "role",
                                    checked: role() == Role::Engineering,
                                    onchange: move |_| role.set(Role::Engineering),
                                }
                                span { class: "role-label", "Engineer (SDE)" }
                            }
                            label {
                                class: if role() == Role::Science { "role-option selected" } else { "role-option" },
                                input {
                                    r#type: "radio",
                                    name: "role",
                                    checked: role() == Role::Science,
                                    onchange: move |_| role.set(Role::Science),
                                }
                                span { class: "role-label", "Scientist (AS)" }
                            }
                        }
                    }

                    // Capacity field
                    div { class: "form-field",
                        label { class: "form-label", "Capacity (weeks) *" }
                        input {
                            r#type: "number",
                            class: "input",
                            step: "0.5",
                            min: "0.5",
                            placeholder: "{props.default_capacity}",
                            value: "{capacity()}",
                            oninput: move |e| {
                                if let Ok(v) = e.value().parse::<f32>() {
                                    capacity.set(v.max(0.0));
                                }
                            },
                        }
                        div { class: "form-hint", "Default: {props.default_capacity} weeks per quarter" }
                        div { class: "form-error", "{capacity_error()}" }
                        if !capacity_warning().is_empty() {
                            div { class: "form-warning", "{capacity_warning()}" }
                        }
                    }
                }

                // Footer
                div { class: "modal-footer",
                    Button {
                        variant: ButtonVariant::Secondary,
                        onclick: move |_| props.on_cancel.call(()),
                        "Cancel"
                    }
                    Button {
                        variant: ButtonVariant::Primary,
                        onclick: handle_save,
                        if matches!(props.mode, TeamMemberModalMode::Add) {
                            "Add Member"
                        } else {
                            "Save Changes"
                        }
                    }
                }
            }
        }
    }
}
