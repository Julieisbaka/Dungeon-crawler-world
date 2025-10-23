use crate::new_save::NewSaveState;

pub struct SaveMenuState {
    #[allow(dead_code)]
    pub show_menu: bool,
    pub new_save_state: NewSaveState,
    pub in_new_save_menu: bool,
    pub editing_save: Option<String>,
    pub edit_save_name: String,
    pub confirm_delete: bool,
    pub delete_target: Option<String>,
    // Set to true when the top-level Back is clicked; caller can observe and react
    pub back_requested: bool,
}

impl Default for SaveMenuState {
    /// Returns a new `SaveMenuState` with default values for all fields.
    fn default() -> Self {
        Self {
            show_menu: false,
            new_save_state: NewSaveState::default(),
            in_new_save_menu: false,
            editing_save: None,
            edit_save_name: String::new(),
            confirm_delete: false,
            delete_target: None,
            back_requested: false,
        }
    }
}
