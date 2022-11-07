//! Mass (base unit kilogram, kg).

quantity! {
    /// Mass (base unit kilogram, kg).
    quantity: Mass; "mass";
    /// Mass dimension, M (base unit kilogram, kg).
    dimension: ISQ<
        Z0,     // length
        P1,     // mass
        Z0,     // time
        Z0,     // electric current
        Z0,     // thermodynamic temperature
        Z0,     // amount of substance
        Z0>;    // luminous intensity
    units {
        @yottagram: prefix!(yotta) / prefix!(kilo); "Yg", "yottagram", "yottagrams";
        @zettagram: prefix!(zetta) / prefix!(kilo); "Zg", "zettagram", "zettagrams";
        @exagram: prefix!(exa) / prefix!(kilo); "Eg", "exagram", "exagrams";
        @petagram: prefix!(peta) / prefix!(kilo); "Pg", "petagram", "petagrams";
        @teragram: prefix!(tera) / prefix!(kilo); "Tg", "teragram", "teragrams";
        @gigagram: prefix!(giga) / prefix!(kilo); "Gg", "gigagram", "gigagrams";
        @megagram: prefix!(mega) / prefix!(kilo); "Mg", "megagram", "megagrams";
        /// The kilogram is the SI unit of mass. It is defined by taking the fixed numerical value
        /// of the Planck constant *h* to be 6.626 070 15 × 10⁻³⁴ when expressed in the unit J s,
        /// which is equal to kg m² s⁻¹, where the meter and the second are defined in terms of *c*
        /// and ∆*ν*<sub>Cs</sub>.
        @kilogram: prefix!(kilo) / prefix!(kilo); "kg", "kilogram", "kilograms";
        @hectogram: prefix!(hecto) / prefix!(kilo); "hg", "hectogram", "hectograms";
        @decagram: prefix!(deca) / prefix!(kilo); "dag", "decagram", "decagrams";
        @gram: prefix!(none) / prefix!(kilo); "g", "gram", "grams";
        @decigram: prefix!(deci) / prefix!(kilo); "dg", "decigram", "decigrams";
        @centigram: prefix!(centi) / prefix!(kilo); "cg", "centigram", "centigrams";
        @milligram: prefix!(milli) / prefix!(kilo); "mg", "milligram", "milligrams";
        @microgram: prefix!(micro) / prefix!(kilo); "µg", "microgram", "micrograms";
        @nanogram: prefix!(nano) / prefix!(kilo); "ng", "nanogram", "nanograms";
        @picogram: prefix!(pico) / prefix!(kilo); "pg", "picogram", "picograms";
        @femtogram: prefix!(femto) / prefix!(kilo); "fg", "femtogram", "femtograms";
        @attogram: prefix!(atto) / prefix!(kilo); "ag", "attogram", "attograms";
        @zeptogram: prefix!(zepto) / prefix!(kilo); "zg", "zeptogram", "zeptograms";
        @yoctogram: prefix!(yocto) / prefix!(kilo); "yg", "yoctogram", "yoctograms";

        @carat: 2.0_E-4; "ct", "carat", "carats";
        /// Unified atomic mass unit.
        @dalton: 1.660_539_066_60_E-27; "Da", "dalton", "daltons";
        @grain: 6.479_891_E-5; "gr", "grain", "grains";
        @hundredweight_long: 5.080_235_E1; "cwt long", "hundredweight (long)",
            "hundredweight (long)";
        @hundredweight_short: 4.535_924_E1; "cwt short", "hundredweight (short)",
            "hundredweight (short)";
        @ounce: 2.834_952_E-2; "oz", "ounce", "ounces";
        @ounce_troy: 3.110_348_E-2; "oz t", "troy ounce", "troy ounces";
        @pennyweight: 1.555_174_E-3; "dwt", "pennyweight", "pennyweight";
        @pound: 4.535_924_E-1; "lb", "pound", "pounds";
        @pound_troy: 3.732_417_E-1; "lb t", "troy pound", "troy pounds";
        @slug: 1.459_390_E1; "slug", "slug", "slugs";
        @ton_assay: 2.916_667_E-2; "AT", "assay ton", "assay tons";
        @ton_long: 1.016_047_E3; "2240 lb", "long ton", "long tons";
        @ton_short: 9.071_847_E2; "2000 lb", "short ton", "short tons";
        @ton: 1.0_E3; "t", "ton", "tons"; // ton, metric

        @alpha_particle_mass: 6.644_657_335_7_E-27; "m(α)", "alpha particle mass",
            "alpha particle masses";
        @deuteron_mass: 3.343_583_772_4_E-27; "m(deuteron)", "deuteron mass", "deuteron masses";
        @electron_mass: 9.109_383_701_5_E-31; "mₑ", "electron mass", "electron masses";
        @helion_mass: 5.006_412_779_6_E-27; "m(helion)", "helion mass", "helion masses";
        @muon_mass: 1.883_531_627_E-28; "m(muon)", "muon mass", "muon masses";
        @neutron_mass: 1.674_927_498_04_E-27; "m(neutron)", "neutron mass", "neutron masses";
        @proton_mass: 1.672_621_923_69_E-27; "m(proton)", "proton mass", "proton masses";
        @tau_mass: 3.167_54_E-27; "m(τ)", "tau mass", "tau masses";
        @triton_mass: 5.007_356_744_6_E-27; "m(triton)", "triton mass", "triton masses";
        @planck_mass: 2.176_434_E-8; "m(Planck)", "Planck mass", "Planck masses";
    }
}
