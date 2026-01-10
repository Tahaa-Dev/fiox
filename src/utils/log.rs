use std::{
    fs::{File, OpenOptions},
    io::{BufWriter, Error, Write},
    sync::{LazyLock, Mutex},
};

use owo_colors::OwoColorize;
use resext::{CtxResult, ErrCtx, ResExt};

static LOGGER: LazyLock<Option<Mutex<BufWriter<File>>>> = LazyLock::new(|| {
    let args = &*crate::ARGS;

    args.log_file.as_ref().map(|path| {
        let file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(path)
            .better_expect("Failed to open error logging file", 1, true);

        Mutex::new(BufWriter::with_capacity(256, file))
    })
});

#[inline]
pub(crate) fn log_err<E: std::error::Error>(err: &ErrCtx<E>) -> CtxResult<(), Error> {
    if let Some(wtr) = &*LOGGER {
        let mut wtr = wtr
            .lock()
            .map_err(|_| Error::other("Failed to lock"))
            .context("Failed to lock log file")?;

        writeln!(
            wtr,
            "{}\nHint: Try to use `fiux validate <INPUT>` for more information\n\n---\n",
            err
        )
        .context("Failed to write error to log")?;
    } else {
        eprintln!(
            "{}\n{} Try to use {} for more information\n\n{}\n",
            err,
            "Hint:".bright_green(),
            "`fiux validate <INPUT>`".yellow(),
            "---".red()
        );
    }

    Ok(())
}

#[inline]
pub(crate) fn flush_logger(msg: &str) -> CtxResult<(), Error> {
    if let Some(wtr) = &*LOGGER {
        let mut wtr = wtr
            .lock()
            .map_err(|_| Error::other("Failed to lock"))
            .context("Failed to lock logger")?;

        wtr.write(msg.as_bytes()).context("Failed to write status message")?;

        wtr.flush().context("Failed to flush logger")?;
    } else {
        eprintln!("{msg}");
    }
    Ok(())
}
