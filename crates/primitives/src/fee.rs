#[typeshare(swift = "Equatable, Sendable, CaseIterable")]
pub enum FeePriority {
    slow,
    normal,
    fast,
}

#[typeshare(swift = "Equatable, Sendable, CaseIterable")]
pub enum FeeUnitType {
    satVb,
    satB,
    gwei,
}
