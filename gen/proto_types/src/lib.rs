extern crate protify;

#[allow(clippy::all, clippy::pedantic, clippy::nursery)]
pub mod job {
    pub mod v1 {
        include!("guggr.job.v1.rs");
    }
    pub mod types {
        pub mod v1 {
            include!("guggr.job.types.v1.rs");
        }
    }
}

#[allow(clippy::all, clippy::pedantic, clippy::nursery)]
pub mod job_result {
    pub mod v1 {
        include!("guggr.job_result.v1.rs");
    }

    pub mod types {
        pub mod v1 {
            include!("guggr.job_result.types.v1.rs");
        }
    }
}
