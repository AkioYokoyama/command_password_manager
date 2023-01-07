use cli_clipboard::{ClipboardContext, ClipboardProvider};

pub(crate) fn copy(password: String) {
    let mut ctx = ClipboardContext::new().unwrap();
    ctx.set_contents(password.to_owned()).unwrap();
}
