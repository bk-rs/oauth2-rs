#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum EndpointOutputObtainFrom {
    None,
    Build,
    Respond,
}
