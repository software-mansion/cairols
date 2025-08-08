/// A helper trait to be used to downcast arbitrary trait objects.
pub trait DowncastRefUnchecked<'t> {
    type From;
    /// Converts the value of type [`Self::From`] to a reference to [`Self`]
    /// without verifying if the downcast is possible.
    ///
    /// # Safety
    /// The caller must ensure that the value is indeed an instance of [`Self`] and can be downcasted.
    unsafe fn downcast_ref_unchecked(value: Self::From) -> &'t Self;
}
