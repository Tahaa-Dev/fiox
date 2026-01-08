use std::{
    fs::{File, OpenOptions},
    io::{BufWriter, Error, Write},
    sync::{LazyLock, Mutex},
};

use resext::{CtxResult, ErrCtx, ResExt};

static LOGGER: LazyLock<Option<Mutex<BufWriter<File>>>> = LazyLock::new(|| {
    let args = &*crate::ARGS;

    args.log_file.as_ref().map(|path| {
        let file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(path)
            .better_expect("FATAL: Failed to open error logging file", 1, true);

        Mutex::new(BufWriter::with_capacity(256, file))
    })
});

#[inline]
pub(crate) fn log_err<E: std::error::Error>(err: &ErrCtx<E>) -> CtxResult<(), Error> {
    if let Some(wtr) = &*LOGGER {
        let mut wtr = wtr
            .lock()
            .map_err(|_| Error::other("Failed to lock"))
            .context("FATAL: Failed to lock log file")?;

        writeln!(
            wtr,
            "{}\nHint: Try to use `fiux validate <INPUT>` for more information\n\n---\n",
            err
        )
        .context("FATAL: Failed to write error to log")?;
    } else {
        eprintln!(
            "{}\nHint: Try to use `fiux validate <INPUT>` for more information\n\n---\n",
            err
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
            .context("FATAL: Failed to lock logger")?;

        wtr.write(msg.as_bytes()).context("FATAL: Failed to write status message")?;

        wtr.flush().context("FATAL: Failed to flush logger")?;
    } else {
        eprintln!("{msg}");
    }
    Ok(())
}
