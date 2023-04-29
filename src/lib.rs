pub mod proto {
    pub mod types {
        include!(concat!(env!("OUT_DIR"), "/proto.types.rs"));
    }
}