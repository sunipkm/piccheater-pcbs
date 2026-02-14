
#[derive(Copy, Clone)]
pub enum FieldId {
    VinMin,
    VinMax,
    VoutMin,
    VoutMax,
    ILoadMax,
    FswKHz,
    RipplePct,
    VinRippleVpp,
    VoutRippleVpp,
    RdsHighmOhm,
    RdsLowmOhm,
    DcrmOhm,
    TrNs,
    TfNs,
    TloadMicrosecond,
    VinDroopVolt,
    Steps,
}

impl FieldId {
    pub fn all() -> &'static [FieldId] {
        &[
            FieldId::VinMin,
            FieldId::VinMax,
            FieldId::VoutMin,
            FieldId::VoutMax,
            FieldId::ILoadMax,
            FieldId::FswKHz,
            FieldId::RipplePct,
            FieldId::VinRippleVpp,
            FieldId::VoutRippleVpp,
            FieldId::RdsHighmOhm,
            FieldId::RdsLowmOhm,
            FieldId::DcrmOhm,
            FieldId::TrNs,
            FieldId::TfNs,
            FieldId::TloadMicrosecond,
            FieldId::VinDroopVolt,
            FieldId::Steps,
        ]
    }

    pub fn label(self) -> &'static str {
        match self {
            FieldId::VinMin => "VIN min (V)",
            FieldId::VinMax => "VIN max (V)",
            FieldId::VoutMin => "VOUT min (V)",
            FieldId::VoutMax => "VOUT max (V)",
            FieldId::ILoadMax => "ILOAD max (A)",
            FieldId::FswKHz => "FSW (kHz)",
            FieldId::RipplePct => "Ripple ratio (%)",
            FieldId::VinRippleVpp => "VIN ripple (Vpp)",
            FieldId::VoutRippleVpp => "VOUT ripple (Vpp)",
            FieldId::RdsHighmOhm => "Rds high (mΩ)",
            FieldId::RdsLowmOhm => "Rds low (mΩ)",
            FieldId::DcrmOhm => "Inductor DCR (mΩ)",
            FieldId::TrNs => "FET Rise Time (ns)",
            FieldId::TfNs => "FET Fall Time (ns)",
            FieldId::TloadMicrosecond => "Load transient (µs)",
            FieldId::VinDroopVolt => "VIN droop (V)",
            FieldId::Steps => "Sweep steps (N)",
        }
    }
}