use druid::{AppDelegate, Command, DelegateCtx, Env, Handled, Selector, Target};

use crate::data::AppState;

pub const SAVE: Selector = Selector::new("todo.save");

pub struct Delegate;

impl AppDelegate<AppState> for Delegate {
    fn command(
        &mut self,
        _ctx: &mut DelegateCtx,
        _target: Target,
        cmd: &Command,
        data: &mut AppState,
        _env: &Env,
    ) -> Handled {
        if cmd.is(SAVE) {
            data.save_to_json().unwrap();
            Handled::Yes
        } else {
            println!("cmd forwarded: {:?}", cmd);
            Handled::No
        }
    }
}