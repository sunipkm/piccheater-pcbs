use crate::{buck_boost_designer::{BuckBoostDesigner, CachedWorst, SweepMap}, fields::FieldId};


pub struct AppState {
    pub selected: usize,
    pub editing: bool,
    pub edit_buf: String,
    pub status: String,
    pub show_heatmap: bool,

    // raw displayed values (strings)
    values: Vec<String>,

    // computed state
    pub designer: BuckBoostDesigner,
    pub steps: usize,
    cached_map: SweepMap,
    pub cached_worst: CachedWorst,
}

impl AppState {
    pub fn new() -> Self {
        // Defaults match earlier examples
        let values = vec![
            "18.0".into(),  // vin_min
            "32.0".into(),  // vin_max
            "3.0".into(),   // vout_min
            "28.0".into(),  // vout_max
            "0.8".into(),   // iload_max
            "450.0".into(), // fsw kHz
            "20.0".into(),  // ripple %
            "0.1".into(),   // vin ripple
            "0.1".into(),   // vout ripple
            "15.0".into(),  // rds high mΩ
            "8.0".into(),   // rds low mΩ
            "12.0".into(),  // dcr mΩ
            "20.0".into(),  // tr ns
            "20.0".into(),  // tf ns
            "50.0".into(),  // tload µs
            "0.5".into(),   // vin droop V
            "25".into(),    // steps
        ];

        let mut s = AppState {
            selected: 0,
            editing: false,
            edit_buf: String::new(),
            status: "↑/↓ select, (Enter) edit/commit, (Esc) cancel, (h) heatmap, (e) export CSV, (m) export Markdown, (q) quit"
                .into(),
            show_heatmap: false,
            values,
            designer: BuckBoostDesigner::default(),
            steps: 25,
            cached_map: SweepMap::default(),
            cached_worst: CachedWorst::default(),
        };

        s.recompute();
        s
    }

    pub fn field_count(&self) -> usize {
        FieldId::all().len()
    }

    pub fn recompute(&mut self) {
        match self.parse_into_designer() {
            Ok(()) => {
                self.cached_map = SweepMap::generate(&self.designer, self.steps);
                self.cached_worst = CachedWorst::from(&self.cached_map);
                self.status =
                    "Recomputed. (h) heatmap, (e) export CSV, (m) export Markdown, (q) quit".into();
            }
            Err(e) => {
                self.status = format!("Input error: {e}");
            }
        }
    }

    fn parse_f64(s: &str) -> Result<f64, String> {
        s.trim()
            .parse::<f64>()
            .map_err(|_| format!("Could not parse '{s}'"))
    }

    fn parse_usize(s: &str) -> Result<usize, String> {
        s.trim()
            .parse::<usize>()
            .map_err(|_| format!("Could not parse '{s}'"))
    }

    pub fn parse_into_designer(&mut self) -> Result<(), String> {
        let v = &self.values;

        let vin_min_v = Self::parse_f64(&v[0])?;
        let vin_max_v = Self::parse_f64(&v[1])?;
        let vout_min_v = Self::parse_f64(&v[2])?;
        let vout_max_v = Self::parse_f64(&v[3])?;
        let iload_a = Self::parse_f64(&v[4])?;
        let fsw_khz = Self::parse_f64(&v[5])?;
        let ripple_pct = Self::parse_f64(&v[6])?;
        let vin_rip = Self::parse_f64(&v[7])?;
        let vout_rip = Self::parse_f64(&v[8])?;
        let rds_high_mohm = Self::parse_f64(&v[9])?;
        let rds_low_mohm = Self::parse_f64(&v[10])?;
        let dcr_mohm = Self::parse_f64(&v[11])?;
        let tr_ns = Self::parse_f64(&v[12])?;
        let tf_ns = Self::parse_f64(&v[13])?;
        let tload_us = Self::parse_f64(&v[14])?;
        let vin_droop_v = Self::parse_f64(&v[15])?;
        let steps = Self::parse_usize(&v[16])?;

        if vin_min_v <= 0.0 || vin_max_v <= 0.0 || vout_min_v <= 0.0 || vout_max_v <= 0.0 {
            return Err("Voltages must be > 0".into());
        }
        if vin_max_v < vin_min_v {
            return Err("VIN max must be >= VIN min".into());
        }
        if vout_max_v < vout_min_v {
            return Err("VOUT max must be >= VOUT min".into());
        }
        if iload_a <= 0.0 {
            return Err("ILOAD must be > 0".into());
        }
        if fsw_khz <= 0.0 {
            return Err("FSW must be > 0".into());
        }
        if vin_rip <= 0.0 || vout_rip <= 0.0 {
            return Err("Ripple specs must be > 0".into());
        }
        if !(1..=200).contains(&steps) {
            return Err("Steps should be between 1 and 200".into());
        }
        if tload_us <= 0.0 {
            return Err("Load transient time must be > 0".into());
        }
        if vin_droop_v < 0.0 {
            return Err("VIN droop must be >= 0".into());
        }

        self.steps = steps;

        self.designer = BuckBoostDesigner::default()
            .with_vin_range(vin_min_v, vin_max_v)
            .with_vout_range(vout_min_v, vout_max_v)
            .with_iload_max(iload_a)
            .with_fsw(fsw_khz)
            .with_ripple_ratio(ripple_pct / 100.0)
            .with_vin_ripple(vin_rip)
            .with_vout_ripple(vout_rip)
            .with_rds_high(rds_high_mohm)
            .with_rds_low(rds_low_mohm)
            .with_dcr(dcr_mohm)
            .with_tr(tr_ns)
            .with_tf(tf_ns)
            .with_tload(tload_us)
            .with_vin_droop(vin_droop_v);

        Ok(())
    }

    pub fn begin_edit(&mut self) {
        self.editing = true;
        self.edit_buf = self.values[self.selected].clone();
        self.status = "Editing: type, Backspace; Enter commit; Esc cancel".into();
    }

    pub fn commit_edit(&mut self) {
        self.values[self.selected] = self.edit_buf.clone();
        self.editing = false;
        self.edit_buf.clear();
        self.recompute();
    }

    pub fn cancel_edit(&mut self) {
        self.editing = false;
        self.edit_buf.clear();
        self.status = "Edit cancelled.".into();
    }

    pub fn selected_value_display(&self, idx: usize) -> String {
        if self.editing && idx == self.selected {
            self.edit_buf.clone()
        } else {
            self.values[idx].clone()
        }
    }
}