//! Mass (base unit kilogram, kg^(1)).

quantity! {
    /// Mass (base unit kilogram, kg^(1)).
    quantity: Mass;
    /// Mass dimension, kg^(1).
    dimension: ISQ<
        Z0,     // length
        P1,     // mass
        Z0,     // time
        Z0,     // electric current
        Z0,     // thermodynamic temperature
        Z0,     // amount of substance
        Z0>;    // luminous intensity
    units {
        @yottagram: prefix!(yotta) / prefix!(kilo);
        @zettagram: prefix!(zetta) / prefix!(kilo);
        @exagram: prefix!(exa) / prefix!(kilo);
        @petagram: prefix!(peta) / prefix!(kilo);
        @teragram: prefix!(tera) / prefix!(kilo);
        @megagram: prefix!(mega) / prefix!(kilo);
        /// The kilogram is the unit of mass; it is equal to the mass of the international prototype
        /// of the kilogram.
        @kilogram: prefix!(kilo) / prefix!(kilo);
        @hectogram: prefix!(hecto) / prefix!(kilo);
        @decagram: prefix!(deca) / prefix!(kilo);
        @gram: prefix!(none) / prefix!(kilo);
        @decigram: prefix!(deci) / prefix!(kilo);
        @centigram: prefix!(centi) / prefix!(kilo);
        @milligram: prefix!(milli) / prefix!(kilo);
        @microgram: prefix!(micro) / prefix!(kilo);
        @nanogram: prefix!(nano) / prefix!(kilo);
        @picogram: prefix!(pico) / prefix!(kilo);
        @femtogram: prefix!(femto) / prefix!(kilo);
        @attogram: prefix!(atto) / prefix!(kilo);
        @zeptogram: prefix!(zepto) / prefix!(kilo);
        @yoctogram: prefix!(yocto) / prefix!(kilo);
    }
}
