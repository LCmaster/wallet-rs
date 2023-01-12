pub trait AppStateLogic {
    fn run(&self) -> AppState;
}

pub enum AppState {
    ExecuteLogic(Box<dyn AppStateLogic>),
    Quit
}

impl AppState {
    pub fn exec(state: AppState) -> Option<AppState> {
        match state {
            Self::ExecuteLogic(logic) => Some(logic.run()),
            Self::Quit => None
        }
    }
}
