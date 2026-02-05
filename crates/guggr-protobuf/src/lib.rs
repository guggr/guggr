extern crate protocheck;

pub mod job {
    pub mod v1 {
        include!("gen/proto/guggr.job.v1.rs");
    }
    pub mod types {
        pub mod v1 {
            include!("gen/proto/guggr.job.types.v1.rs");
        }
    }
}

pub mod job_result {
    pub mod v1 {
        include!("gen/proto/guggr.job_result.v1.rs");
    }

    pub mod types {
        pub mod v1 {
            include!("gen/proto/guggr.job_result.types.v1.rs");
        }
    }
}
