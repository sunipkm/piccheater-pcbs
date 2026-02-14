use chrono::Local;
use uom::{
    ConstZero,
    si::{
        capacitance::microfarad,
        electric_current::ampere,
        electric_potential::volt,
        electrical_resistance::milliohm,
        f64::{
            Capacitance, ElectricCurrent, ElectricPotential, ElectricalResistance, Frequency,
            Inductance, Ratio, Time,
        },
        frequency::kilohertz,
        inductance::microhenry,
        time::{microsecond, nanosecond},
    },
};

use std::error::Error;

#[derive(Clone)]
pub struct BuckBoostDesigner {
    vin_min: ElectricPotential,
    vin_max: ElectricPotential,
    vout_min: ElectricPotential,
    vout_max: ElectricPotential,
    iload_max: ElectricCurrent,
    fsw: Frequency,
    ripple_ratio: Ratio,
    vin_ripple_pp: ElectricPotential,
    vout_ripple_pp: ElectricPotential,
    rds_high: ElectricalResistance,
    rds_low: ElectricalResistance,
    dcr: ElectricalResistance,
    tr: Time,
    tf: Time,
    tload: Time,
    vin_droop: ElectricPotential,
}

impl Default for BuckBoostDesigner {
    fn default() -> Self {
        Self {
            vin_min: ElectricPotential::new::<volt>(18.0),
            vin_max: ElectricPotential::new::<volt>(32.0),
            vout_min: ElectricPotential::new::<volt>(3.0),
            vout_max: ElectricPotential::new::<volt>(28.0),
            iload_max: ElectricCurrent::new::<ampere>(0.8),
            fsw: Frequency::new::<kilohertz>(450.0),
            ripple_ratio: Ratio::from(0.2),
            vin_ripple_pp: ElectricPotential::new::<volt>(0.1),
            vout_ripple_pp: ElectricPotential::new::<volt>(0.1),
            rds_high: ElectricalResistance::new::<milliohm>(12.0),
            rds_low: ElectricalResistance::new::<milliohm>(5.0),
            dcr: ElectricalResistance::new::<milliohm>(100.0),
            tr: Time::new::<nanosecond>(50.0),
            tf: Time::new::<nanosecond>(50.0),
            tload: Time::new::<microsecond>(100.0),
            vin_droop: ElectricPotential::new::<volt>(0.5),
        }
    }
}

impl BuckBoostDesigner {
    /// Set the range of input voltages (in Volts).
    ///
    /// Note: This is a builder-style function.
    pub fn with_vin_range(mut self, vin_min: f64, vin_max: f64) -> Self {
        self.vin_min = ElectricPotential::new::<volt>(vin_min);
        self.vin_max = ElectricPotential::new::<volt>(vin_max);
        self
    }

    /// Set the range of output voltages (in Volts).
    ///
    /// Note: This is a builder-style function.
    pub fn with_vout_range(mut self, vout_min: f64, vout_max: f64) -> Self {
        self.vout_min = ElectricPotential::new::<volt>(vout_min);
        self.vout_max = ElectricPotential::new::<volt>(vout_max);
        self
    }

    /// Set the maximum load current (in Amperes).
    ///
    /// Note: This is a builder-style function.
    pub fn with_iload_max(mut self, iload_max: f64) -> Self {
        self.iload_max = ElectricCurrent::new::<ampere>(iload_max);
        self
    }

    /// Set the nominal switching frequency (kHz).
    ///
    /// Note: This is a builder-style function.
    pub fn with_fsw(mut self, fsw: f64) -> Self {
        self.fsw = Frequency::new::<kilohertz>(fsw);
        self
    }

    /// Set the inductor current ripple ratio (%).
    /// This is a percentage of the maximum load current.
    ///
    /// Note: This is a builder-style function.
    pub fn with_ripple_ratio(mut self, ripple_ratio: f64) -> Self {
        self.ripple_ratio = Ratio::from(ripple_ratio);
        self
    }

    /// Set the ripple on the input voltage (V).
    ///
    /// Note: This is a builder-style function.
    pub fn with_vin_ripple(mut self, vin_ripple_pp: f64) -> Self {
        self.vin_ripple_pp = ElectricPotential::new::<volt>(vin_ripple_pp);
        self
    }

    /// Set the ripple on the output voltage (V).
    ///
    /// Note: This is a builder-style function.
    pub fn with_vout_ripple(mut self, vout_ripple_pp: f64) -> Self {
        self.vout_ripple_pp = ElectricPotential::new::<volt>(vout_ripple_pp);
        self
    }

    /// Set the highest drain-source resistance on the switching FETs (milliohms).
    ///
    /// Note: This is a builder-style function.
    pub fn with_rds_high(mut self, rds_high: f64) -> Self {
        self.rds_high = ElectricalResistance::new::<milliohm>(rds_high);
        self
    }

    /// Set the lowest drain-source resistance on the switching FETs (milliohms).
    ///
    /// Note: This is a builder-style function.
    pub fn with_rds_low(mut self, rds_low: f64) -> Self {
        self.rds_low = ElectricalResistance::new::<milliohm>(rds_low);
        self
    }

    /// Set the DC resistance of the inductor (milliohms).
    ///
    /// Note: This is a builder-style function.
    pub fn with_dcr(mut self, dcr: f64) -> Self {
        self.dcr = ElectricalResistance::new::<milliohm>(dcr);
        self
    }

    /// Set the FET rise time (ns)
    ///
    /// Note: This is a builder-style function.
    pub fn with_tr(mut self, tr: f64) -> Self {
        self.tr = Time::new::<nanosecond>(tr);
        self
    }

    /// Set the FET fall time (ns)
    ///
    /// Note: This is a builder-style function.
    pub fn with_tf(mut self, tf: f64) -> Self {
        self.tf = Time::new::<nanosecond>(tf);
        self
    }

    /// Set the load transient time (Vout 0 to max)
    ///
    /// Note: This is a builder-style function.
    pub fn with_tload(mut self, tload: f64) -> Self {
        self.tload = Time::new::<microsecond>(tload);
        self
    }

    /// Set acceptable droop in Vin from max Vin
    ///
    /// Note: This is a builder-style function.
    pub fn with_vin_droop(mut self, vin_droop: f64) -> Self {
        self.vin_droop = ElectricPotential::new::<volt>(vin_droop);
        self
    }

    /// Calculate the duty cycle
    fn duty(&self, vin: ElectricPotential, vout: ElectricPotential) -> Ratio {
        if vin > vout {
            // buck mode
            vout / vin
        } else {
            // boost mode
            Ratio::from(1.0) - (vin / vout)
        }
    }

    // AN5890 Eq. 3 & 4
    fn inductor_value(&self, vin: ElectricPotential, vout: ElectricPotential) -> Inductance {
        // ripple current
        let delta_i = self.iload_max * self.ripple_ratio;

        if vin > vout {
            (vout * (vin - vout)) / (delta_i * self.fsw * vin)
        } else {
            (vin * (vout - vin)) / (delta_i * self.fsw * vout)
        }
    }

    // AN5890 Eq. 7 & 8
    fn inductor_ripple(
        &self,
        vin: ElectricPotential,
        vout: ElectricPotential,
        l: Inductance,
    ) -> ElectricCurrent {
        if vin > vout {
            (vout * (vin - vout)) / (l * self.fsw * vin)
        } else {
            (vin * (vout - vin)) / (l * self.fsw * vout)
        }
    }

    // AN5890 Eq. 5 & 6
    fn peak_current(
        &self,
        vin: ElectricPotential,
        vout: ElectricPotential,
        l: Inductance,
    ) -> ElectricCurrent {
        let delta_i = self.inductor_ripple(vin, vout, l);

        let i_avg = if vin > vout {
            self.iload_max
        } else {
            self.iload_max * (vout / vin)
        };

        i_avg + delta_i / 2.0
    }

    fn efficiency(&self, vin: ElectricPotential, vout: ElectricPotential, l: Inductance) -> Ratio {
        let pout = vout * self.iload_max;

        let i_peak = self.peak_current(vin, vout, l);
        let i_rms = i_peak / f64::sqrt(3.0);

        let conduction = i_rms * i_rms * (self.rds_high + self.rds_low + self.dcr);

        let switching = 0.5 * vin * self.iload_max * (self.tr + self.tf) * self.fsw;

        pout / (pout + conduction + switching)
    }

    // AN5890 Eq. 9 & 10
    fn cin_required(&self, vin: ElectricPotential, vout: ElectricPotential) -> Capacitance {
        let d = self.duty(vin, vout);

        if vin > vout {
            self.iload_max * d * (Ratio::from(1.0) - d) / (self.fsw * self.vin_ripple_pp)
        } else {
            self.iload_max * self.ripple_ratio
                / (8.0 * self.fsw * self.vin_ripple_pp * (Ratio::from(1.0) - d))
        }
    }

    // AN5890 Eq. 11, 12 & 13
    fn cin_bulk(&self, vin: ElectricPotential) -> Capacitance {
        let pow = self.iload_max * self.vout_max * self.tload;
        let vin_dropped = vin - self.vin_droop;
        2.0 * pow / (vin * vin - vin_dropped * vin_dropped)
    }

    /// Minimum bulk capacitor required to
    /// prevent input voltage droop (at max
    /// input voltage) for load switching
    /// from off to on.
    pub fn cin_bulk_min(&self) -> Capacitance {
        self.cin_bulk(self.vin_max)
    }

    /// Maximum bulk capacitor required to
    /// prevent input voltage droop (at min
    /// input voltage) for load switching
    /// from off to on.
    pub fn cin_bulk_max(&self) -> Capacitance {
        self.cin_bulk(self.vin_min)
    }

    // AN5890 Eq. 15 & 18
    fn cout_required(
        &self,
        vin: ElectricPotential,
        vout: ElectricPotential,
        l: Inductance,
    ) -> Capacitance {
        let delta_i = self.inductor_ripple(vin, vout, l);

        if vin > vout {
            delta_i / (8.0 * self.fsw * self.vout_ripple_pp)
        } else {
            let d = self.duty(vin, vout);
            self.iload_max * (Ratio::from(1.0) - d) / (self.fsw * self.vout_ripple_pp)
        }
    }

    /// Export the map in [`generate_map`] as a CSV file.
    pub fn export_csv(&self, filename: &str, steps: usize) -> Result<(), Box<dyn Error>> {
        let map = SweepMap::generate(self, steps);
        let mut wtr = csv::Writer::from_path(filename)?;

        wtr.write_record([
            "VIN (V)",
            "VOUT (V)",
            "I_peak (A)",
            "CIN (uF)",
            "COUT (uF)",
            "Efficiency (%)",
        ])?;

        for i in 0..steps {
            wtr.write_record(&[
                map.vin[i].get::<volt>().to_string(),
                map.vout[i].get::<volt>().to_string(),
                map.peak_current[i].get::<ampere>().to_string(),
                map.cin[i].get::<microfarad>().to_string(),
                map.cout[i].get::<microfarad>().to_string(),
                map.efficiency[i].value.to_string(),
            ])?;
        }

        wtr.flush()?;
        Ok(())
    }

    /// Export the worst-case scenario design parameters ([`worst_from_map`])
    /// to a Markdown file.
    pub fn render_markdown(
        &self,
        filename: &str,
        tstamp: chrono::DateTime<Local>,
    ) -> Result<(), Box<dyn Error>> {
        let mut file = std::fs::File::create(filename)?;

        use std::io::Write;
        writeln!(
            file,
            "# MCP19061 Buck-Boost Design Map ({})",
            tstamp.format("%Y-%m-%d %H:%M")
        )?;
        writeln!(file, "## Input Design Parameters")?;
        writeln!(file, "| Parameter | Value | Description |")?;
        writeln!(file, "|-----------|-------|-------------|")?;
        writeln!(
            file,
            "| V<sub>in</sub> (min) | {:.2} V | Minimum input voltage |",
            self.vin_min.get::<volt>()
        )?;
        writeln!(
            file,
            "| V<sub>in</sub> (max) | {:.2} V | Maximum input voltage |",
            self.vin_max.get::<volt>()
        )?;
        writeln!(
            file,
            "| V<sub>out</sub> (min) | {:.2} V | Minimum output voltage |",
            self.vout_min.get::<volt>()
        )?;
        writeln!(
            file,
            "| V<sub>out</sub> (max) | {:.2} V | Maximum output voltage |",
            self.vout_max.get::<volt>()
        )?;
        writeln!(
            file,
            "| I<sub>load</sub> (max) | {:.2} A | Maximum load current |",
            self.iload_max.get::<ampere>()
        )?;
        writeln!(
            file,
            "| Ripple Ratio | {:.2} % | Desired inductor current ripple ratio to max load current |",
            self.ripple_ratio.value * 100.0
        )?;
        writeln!(
            file,
            "│ V<sub>in</sub> Ripple | {:.2} V | Allowed input voltage ripple |",
            self.vin_ripple_pp.get::<volt>()
        )?;
        writeln!(
            file,
            "│ V<sub>out</sub> Ripple | {:.2} V | Allowed output voltage ripple |",
            self.vout_ripple_pp.get::<volt>()
        )?;
        writeln!(
            file,
            "| Load Transient | {:.2} µs | Expected transition duration from 0 to full load |",
            self.tload.get::<microsecond>()
        )?;
        writeln!(
            file,
            "| V<sub>in</sub> Droop (V) | {:.2} V | Expected input voltage droop during transition |",
            self.vin_droop.get::<volt>()
        )?;
        writeln!(
            file,
            "| Inductor DCR | {:.2} mΩ | Inductor DC resistance |",
            self.dcr.get::<milliohm>()
        )?;
        writeln!(
            file,
            "| R<sub>ds,on</sub> High | {:.2} mΩ | High-side FET on-resistance |",
            self.rds_high.get::<milliohm>()
        )?;
        writeln!(
            file,
            "| R<sub>ds,on</sub> Low | {:.2} mΩ | Low-side FET on-resistance |",
            self.rds_low.get::<milliohm>()
        )?;
        writeln!(
            file,
            "| Rise Time (ns) | {:.2} ns | FET rise time |",
            self.tr.get::<nanosecond>()
        )?;
        writeln!(
            file,
            "| Fall Time (ns) | {:.2} ns | FET fall time |",
            self.tf.get::<nanosecond>()
        )?;
        writeln!(
            file,
            "| Switching Frequency | {:.2} kHz | Desired switching frequency |",
            self.fsw.get::<kilohertz>()
        )?;

        writeln!(file, "## Design Map")?;

        writeln!(file, "| Parameter | Value | Description |")?;
        writeln!(file, "|-----------|-------|-------------|")?;

        let map = SweepMap::generate(self, 25);
        let worst = CachedWorst::from(&map);
        writeln!(
            file,
            "| Inductance | {:.2} µH | Maximum inductance required |",
            worst.inductance.get::<microhenry>()
        )?;
        writeln!(
            file,
            "| Peak Inductor Current | {:.2} A | Maximum peak current through inductor |",
            worst.peak_current.get::<ampere>()
        )?;
        writeln!(
            file,
            "| Input Capacitance | {:.2} µF | Maximum required input capacitance |",
            worst.cin.get::<microfarad>()
        )?;
        writeln!(
            file,
            "| Input Capacitance (bulk, min) | {:.2} µF | Required bulk input capacitance for load transient |",
            self.cin_bulk_min().get::<microfarad>()
        )?;
        writeln!(
            file,
            "| Input Capacitance (bulk, max) | {:.2} µF | Required bulk input capacitance for load transient |",
            self.cin_bulk_max().get::<microfarad>()
        )?;
        writeln!(
            file,
            "| Output Capacitance | {:.2} µF | Maximum required output capacitance |",
            worst.cout.get::<microfarad>()
        )?;
        writeln!(
            file,
            "| Efficiency (min) | {:.2} % | Minimum efficiency over VIN×VOUT sweep |",
            worst.eff_low.value * 100.0
        )?;
        writeln!(
            file,
            "| Efficiency (max) | {:.2} % | Maximum efficiency over VIN×VOUT sweep |",
            worst.eff_high.value * 100.0
        )?;
        writeln!(
            file,
            "| FET Peak Current (Buck) | {:.2} A | Maximum FET current in buck mode |",
            worst.fet_peak_buck.get::<ampere>()
        )?;
        writeln!(
            file,
            "| FET Peak Current (Boost) | {:.2} A | Maximum FET current in boost mode |",
            worst.fet_peak_boost.get::<ampere>()
        )?;

        Ok(())
    }

    /// Generate an ASCII art Heat Map
    pub fn ascii_heatmap(&self, steps: usize, inductance: Inductance) -> String {
        let mut output = String::new();

        for i in 0..=steps {
            let vin = self.vin_min + (self.vin_max - self.vin_min) * (i as f64 / steps as f64);

            for j in 0..=steps {
                let vout =
                    self.vout_min + (self.vout_max - self.vout_min) * (j as f64 / steps as f64);

                let eff = self.efficiency(vin, vout, inductance);

                let e = eff.value;
                let symbol = match e {
                    e if e < 0.70 => ' ',
                    e if e < 0.75 => '.',
                    e if e < 0.80 => '-',
                    e if e < 0.85 => '=',
                    e if e < 0.90 => '+',
                    e if e < 0.93 => '*',
                    _ => '#',
                };

                output.push(symbol);
            }

            output.push('\n');
        }

        output
    }
}

#[derive(Copy, Clone, Default)]
/// Worst-case scenario design parameters
pub struct CachedWorst {
    /// Inductance
    pub inductance: Inductance,
    /// Peak inductor current
    pub peak_current: ElectricCurrent,
    /// Input capacitance
    pub cin: Capacitance,
    /// Output capacitance
    pub cout: Capacitance,
    /// Highest efficiency
    pub eff_high: Ratio,
    /// Lowest efficiency
    pub eff_low: Ratio,
    /// Peak current through Buck FETs
    pub fet_peak_buck: ElectricCurrent,
    /// Peak current through Boost FETs
    pub fet_peak_boost: ElectricCurrent,
}

#[derive(Clone, Default)]
/// Swept map of efficiency and peak current
pub struct SweepMap {
    pub inductance: Inductance,
    pub fet_peak_buck: ElectricCurrent,
    pub fet_peak_boost: ElectricCurrent,
    pub vin: Vec<ElectricPotential>,
    pub vout: Vec<ElectricPotential>,
    pub peak_current: Vec<ElectricCurrent>,
    pub cin: Vec<Capacitance>,
    pub cout: Vec<Capacitance>,
    pub efficiency: Vec<Ratio>,
}

impl SweepMap {
    /// Generate a sweep map from a model
    ///
    /// # Inputs
    /// - model: [`BuckBoostDesigner`] model.
    /// - steps: Number of steps to sweep in Vin and Vout
    pub fn generate(model: &BuckBoostDesigner, steps: usize) -> Self {
        let dv_in = (model.vin_max - model.vin_min) / steps as f64;
        let dv_out = (model.vout_max - model.vout_min) / steps as f64;
        let mut lmax = Inductance::ZERO;

        let mut vin = Vec::with_capacity(steps);
        let mut vout = Vec::with_capacity(steps);
        for i in 0..=steps {
            let vin_v = model.vin_min + dv_in * (i as f64);

            for j in 0..=steps {
                let vout_v = model.vout_min + dv_out * (j as f64);

                let l = model.inductor_value(vin_v, vout_v);
                if l > lmax {
                    lmax = l;
                }
                vin.push(vin_v);
                vout.push(vout_v);
            }
        }

        // Equation 7 & 8 from AN5890, using worst-case ripple for peak current
        let fet_peak_buck =
            model.iload_max + model.inductor_ripple(model.vin_min, model.vout_max, lmax) / 2.0;
        let fet_peak_boost = model.iload_max * (model.vout_max / model.vin_min)
            + model.inductor_ripple(model.vin_min, model.vout_max, lmax) / 2.0;

        let mut cin = Vec::with_capacity(steps);
        let mut cout = Vec::with_capacity(steps);
        let mut peak_current = Vec::with_capacity(steps);
        let mut efficiency = Vec::with_capacity(steps);

        for (vin_v, vout_v) in vin.iter().zip(&vout) {
            peak_current.push(model.peak_current(*vin_v, *vout_v, lmax));
            cin.push(model.cin_required(*vin_v, *vout_v));
            cout.push(model.cout_required(*vin_v, *vout_v, lmax));
            efficiency.push(model.efficiency(*vin_v, *vout_v, lmax));
        }

        Self {
            inductance: lmax,
            fet_peak_buck,
            fet_peak_boost,
            vin,
            vout,
            peak_current,
            cin,
            cout,
            efficiency,
        }
    }
}

impl From<&SweepMap> for CachedWorst {
    fn from(value: &SweepMap) -> Self {
        let mut worst_eff = Ratio::from(1.0);
        let mut best_eff = Ratio::from(0.0);

        let worst_i = value
            .peak_current
            .iter()
            .max_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
            .unwrap_or(&ElectricCurrent::ZERO);
        let worst_cin = value
            .cin
            .iter()
            .max_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
            .unwrap_or(&Capacitance::ZERO);
        let worst_cout = value
            .cout
            .iter()
            .max_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
            .unwrap_or(&Capacitance::ZERO);
        for eff in value.efficiency.iter() {
            let eff = *eff;
            if eff > best_eff {
                best_eff = eff;
            }
            if eff < worst_eff {
                worst_eff = eff;
            }
        }

        Self {
            inductance: value.inductance,
            peak_current: *worst_i,
            cin: *worst_cin,
            cout: *worst_cout,
            eff_high: best_eff,
            eff_low: worst_eff,
            fet_peak_buck: value.fet_peak_buck,
            fet_peak_boost: value.fet_peak_boost,
        }
    }
}
