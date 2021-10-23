/// A [`DeletePolicy`] represents what "rules" should be followed when automatically deleting seemingly duplicate images.
pub enum DeletePolicy {
    /// Keep the oldest file in the case of duplication.
    Oldest,

    /// Keep the newest file in the case of duplication.
    Newest,

    /// Keep the image with the most amount of similar images.
    MostSimilar,

    /// Keep the biggest image in terms of file size.
    Biggest,

    /// Keep the smallest image in terms of file size.
    Smallest,
}
