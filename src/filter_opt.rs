#![macro_use]

macro_rules! filter_opt {
    ($e:expr, $t:expr) =>(
        match $e {
            Some(v) => v,
            None => $t,
        }
    )
}

macro_rules! filter_opt_ok {
    ($e:expr, $t:expr) =>(
        match $e {
            Ok(v) => v,
            Err(_) => $t,
        }
    )
}
