mod sync;
pub mod r#async;

pub trait DbTraits {}

pub trait Id: Copy + Eq {
    type Underlying;

    fn to_underlying(self) -> Self::Underlying;

    fn from_underlying(underlying: Self::Underlying) -> Self;
}

pub trait TrustedId: Copy + Eq {
    type Underlying;

    type Untrusted: UntrustedId<Underlying = Self::Underlying>;

    unsafe fn from_untrusted_unchecked(untrusted: Self::Untrusted) -> Self;

    fn to_untrusted(self) -> Self::Untrusted;

    fn to_underlying(self) -> Self::Underlying;
}

pub trait UntrustedId: Copy + Eq  {
    type Underlying;

    type Trusted: TrustedId<Underlying = Self::Underlying>;

    fn from_underlying(underlying: Self::Underlying) -> Self;

    fn from_trusted(trusted: Self::Trusted) -> Self;

    unsafe fn to_trusted_unchecked(self) -> Self::Trusted;
}


#[cfg(feature = "tokio-postgres")]
pub struct PostgresTypesTraits;

#[cfg(feature = "tokio-postgres")]
impl DbTraits for PostgresTypesTraits {}
