#[cfg(not(feature = "postgres"))]
mod mock;
#[cfg(feature = "postgres")]
mod postgr;

#[cfg(not(feature = "postgres"))]
pub use mock::MockStore;
#[cfg(feature = "postgres")]
pub use postgr::PostgresStore;
