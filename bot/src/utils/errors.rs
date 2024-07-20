#[macro_export]
macro_rules! log_error {
    ($e:expr) => {
        if let Err(e) = $e {
            let e = anyhow::anyhow!(e);
            tracing::error!(
                error.message = %&e,
                error.root_cause = %e.root_cause(),
                "{:?}",
                e
            );
        }
    };
    ($context:expr, $e:expr $(,)?) => {
        if let Err(e) = $e {
            let e = ::anyhow::anyhow!(e).context($context);
            tracing::error!(
                error.message = %&e,
                error.root_cause = %e.root_cause(),
                "{:?}",
                e
            );
        }
    };
}

#[macro_export]
macro_rules! abort_with {
    ($err:literal) => {
        return Err(crate::commands::errors::UserErr::new($err).into())
    };
    ($err:expr) => {
        return Err($err.into())
    };
}

pub(crate) use log_error;

#[allow(unused_imports)]
pub(crate) use abort_with;