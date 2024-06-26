use crate::prelude::*;

use crate::ImvrEventLoop;

#[derive(Debug)]
pub struct WindowError;

impl fmt::Display for WindowError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Window thread enocuntered and error. Likely the result of an OS error.")
    }
}
impl Context for WindowError {}

pub fn window(eloop: ImvrEventLoop) -> Result<(), WindowError> {
    let mut context = GlobalContext::new();

    let res = eloop.run(move |evnt, elwt| {
        if let winit::event::Event::UserEvent(ref e) = evnt {
            log::info!("user event: {:?}", &e);
        }

        let Some(msg) = evnt.some_into() else { return };

        let res = context.handle(msg, elwt);

        if let Err(e) = res {
            log::error!("{e}");
            if e.current_context().is_fatal() {
                elwt.exit();
            }
        }
    });

    res.attach_printable("event loop returned unexpected error.")
        .change_context(WindowError)?;

    log::warn!("Event Loop Ended.");

    Ok(())
}
