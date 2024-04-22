use crate::{
    define_hook,
    sdk::{
        entity::player::Player,
        predictions::{MoveHelper, Prediction},
        user_cmd::UserCmd,
    },
};

fn subhooks(hook: &mut RunCommandHook) {
    hook.before = Some(|_, _, _, move_helper| {
        if o!().engine_prediction.move_helper.is_none() {
            o!().engine_prediction.move_helper = Some(move_helper);
        }
        Ok(None)
    });
    hook.after = Some(|_, _, _, _, _| Ok(()));
}

define_hook!(
    RunCommandHook,
    "RunCommand",
    (),
    (),
    subhooks,
    prediction,
    &Prediction,
    player,
    &Player,
    cmd,
    &UserCmd,
    move_helper,
    &'static MoveHelper
);
