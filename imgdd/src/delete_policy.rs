/// A [`DeletePolicy`] represents what "rules" should be followed when deleting duplicate images.
pub enum DeletePolicy {
    /// Keep the oldest file in the case of duplication.
    Oldest,

    /// Keep the newest file in the case of duplication.
    Newest,
}
