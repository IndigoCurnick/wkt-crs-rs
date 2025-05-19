use std::str::FromStr;

use strum::{AsRefStr, EnumString};

use crate::{
    ast::WktArg,
    coordinate_system::{bearing::Bearing, meridian::Meridian},
    error::WktParseError,
};

#[derive(Debug, PartialEq, EnumString)]
pub enum AxisDirection {
    #[strum(disabled)]
    North(Option<Meridian>),
    #[strum(serialize = "northNorthEast")]
    NorthNorthEast,
    #[strum(serialize = "northEast")]
    NorthEast,
    #[strum(serialize = "eastNorthEast")]
    EastNorthEast,
    #[strum(serialize = "east")]
    East,
    #[strum(serialize = "eastSouthEast")]
    EastSouthEast,
    #[strum(serialize = "southEast")]
    SouthEast,
    #[strum(serialize = "southSouthEast")]
    SouthSouthEast,
    #[strum(disabled)]
    South(Option<Meridian>),
    #[strum(serialize = "southSouthWest")]
    SouthSouthWest,
    #[strum(serialize = "southWest")]
    SouthWest,
    #[strum(serialize = "westSouthWest")]
    WestSouthWest,
    #[strum(serialize = "west")]
    West,
    #[strum(serialize = "westNorthWest")]
    WestNorthWest,
    #[strum(serialize = "northWest")]
    NorthWest,
    #[strum(serialize = "northNorthWest")]
    NorthNorthWest,
    #[strum(serialize = "geocentricX")]
    GeocentricX,
    #[strum(serialize = "geocentricY")]
    GeocentricY,
    #[strum(serialize = "geocentricZ")]
    GeocentricZ,
    #[strum(serialize = "up")]
    Up,
    #[strum(serialize = "down")]
    Down,
    #[strum(serialize = "forward")]
    Forward,
    #[strum(serialize = "aft")]
    Aft,
    #[strum(serialize = "port")]
    Port,
    #[strum(serialize = "starboard")]
    Starboard,
    #[strum(disabled)]
    Clockwise(Bearing),
    #[strum(disabled)]
    CounterClockwise(Bearing),
    #[strum(serialize = "columnPositive")]
    ColumnPositive,
    #[strum(serialize = "columnNegative")]
    ColumnNegative,
    #[strum(serialize = "rowPositive")]
    RowPositive,
    #[strum(serialize = "rowNegative")]
    RowNegative,
    #[strum(serialize = "displayRight")]
    DisplayRight,
    #[strum(serialize = "displayLeft")]
    DisplayLeft,
    #[strum(serialize = "displayUp")]
    DisplayUp,
    #[strum(serialize = "displayDown")]
    DisplayDown,
    #[strum(serialize = "future")]
    Future,
    #[strum(serialize = "past")]
    Past,
    #[strum(serialize = "towards")]
    Towards,
    #[strum(serialize = "awayFrom")]
    AwayFrom,
    #[strum(serialize = "unspecified")]
    Unspecified,
}

#[derive(Debug, PartialEq, EnumString, AsRefStr)]
enum AxisDirectionInner {
    #[strum(serialize = "north")]
    North,
    #[strum(serialize = "northNorthEast")]
    NorthNorthEast,
    #[strum(serialize = "northEast")]
    NorthEast,
    #[strum(serialize = "eastNorthEast")]
    EastNorthEast,
    #[strum(serialize = "east")]
    East,
    #[strum(serialize = "eastSouthEast")]
    EastSouthEast,
    #[strum(serialize = "southEast")]
    SouthEast,
    #[strum(serialize = "southSouthEast")]
    SouthSouthEast,
    #[strum(serialize = "south")]
    South,
    #[strum(serialize = "southSouthWest")]
    SouthSouthWest,
    #[strum(serialize = "southWest")]
    SouthWest,
    #[strum(serialize = "westSouthWest")]
    WestSouthWest,
    #[strum(serialize = "west")]
    West,
    #[strum(serialize = "westNorthWest")]
    WestNorthWest,
    #[strum(serialize = "northWest")]
    NorthWest,
    #[strum(serialize = "northNorthWest")]
    NorthNorthWest,
    #[strum(serialize = "geocentricX")]
    GeocentricX,
    #[strum(serialize = "geocentricY")]
    GeocentricY,
    #[strum(serialize = "geocentricZ")]
    GeocentricZ,
    #[strum(serialize = "up")]
    Up,
    #[strum(serialize = "down")]
    Down,
    #[strum(serialize = "forward")]
    Forward,
    #[strum(serialize = "aft")]
    Aft,
    #[strum(serialize = "port")]
    Port,
    #[strum(serialize = "starboard")]
    Starboard,
    #[strum(serialize = "clockwise")]
    Clockwise,
    #[strum(serialize = "counterClockwise")]
    CounterClockwise,
    #[strum(serialize = "columnPositive")]
    ColumnPositive,
    #[strum(serialize = "columnNegative")]
    ColumnNegative,
    #[strum(serialize = "rowPositive")]
    RowPositive,
    #[strum(serialize = "rowNegative")]
    RowNegative,
    #[strum(serialize = "displayRight")]
    DisplayRight,
    #[strum(serialize = "displayLeft")]
    DisplayLeft,
    #[strum(serialize = "displayUp")]
    DisplayUp,
    #[strum(serialize = "displayDown")]
    DisplayDown,
    #[strum(serialize = "future")]
    Future,
    #[strum(serialize = "past")]
    Past,
    #[strum(serialize = "towards")]
    Towards,
    #[strum(serialize = "awayFrom")]
    AwayFrom,
    #[strum(serialize = "unspecified")]
    Unspecified,
}

impl TryFrom<(&WktArg, Option<&WktArg>)> for AxisDirection {
    type Error = WktParseError;

    fn try_from(value: (&WktArg, Option<&WktArg>)) -> Result<Self, Self::Error> {
        let (definite, maybe) = value;

        let inner = match definite {
            WktArg::String(s) => match AxisDirectionInner::from_str(s) {
                Ok(x) => x,
                Err(y) => return Err(WktParseError::ParseError(y)),
            },
            _ => return Err(WktParseError::ExpectedString),
        };

        return AxisDirection::try_from((inner, maybe));
    }
}

impl TryFrom<(AxisDirectionInner, Option<&WktArg>)> for AxisDirection {
    type Error = WktParseError;

    fn try_from(value: (AxisDirectionInner, Option<&WktArg>)) -> Result<Self, Self::Error> {
        let (inner, node) = value;

        match inner {
            AxisDirectionInner::North => {
                let meridian = match node {
                    Some(arg) => match arg {
                        WktArg::Node(node) => match Meridian::try_from(node) {
                            Ok(x) => Some(x),
                            Err(y) => return Err(y),
                        },
                        _ => return Err(WktParseError::ExpectedNode),
                    },
                    None => None,
                };

                return Ok(AxisDirection::North(meridian));
            }
            AxisDirectionInner::South => {
                let meridian = match node {
                    Some(arg) => match arg {
                        WktArg::Node(node) => match Meridian::try_from(node) {
                            Ok(x) => Some(x),
                            Err(y) => return Err(y),
                        },
                        _ => return Err(WktParseError::ExpectedNode),
                    },
                    None => None,
                };

                return Ok(AxisDirection::South(meridian));
            }
            AxisDirectionInner::Clockwise => {
                let bearing = match node {
                    Some(arg) => match arg {
                        WktArg::Node(node) => match Bearing::try_from(node) {
                            Ok(x) => x,
                            Err(y) => return Err(y),
                        },
                        _ => return Err(WktParseError::ExpectedNode),
                    },
                    None => return Err(WktParseError::ExpectedNode),
                };

                return Ok(AxisDirection::Clockwise(bearing));
            }
            AxisDirectionInner::CounterClockwise => {
                let bearing = match node {
                    Some(arg) => match arg {
                        WktArg::Node(node) => match Bearing::try_from(node) {
                            Ok(x) => x,
                            Err(y) => return Err(y),
                        },
                        _ => return Err(WktParseError::ExpectedNode),
                    },
                    None => return Err(WktParseError::ExpectedNode),
                };

                return Ok(AxisDirection::CounterClockwise(bearing));
            }
            _ => return Ok(AxisDirection::from_str(inner.as_ref()).unwrap()),
        }
    }
}
