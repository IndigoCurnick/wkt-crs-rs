use log::{error, warn};

use crate::{
    ast::{WktArg, WktNode},
    error::WktParseError,
    id::Id,
    keywords::{
        ANGLEUNIT, CONVERSION, ID, LENGTHUNIT, METHOD, PARAMETER, PROJECTION, SCALEUNIT, UNIT,
    },
    units::{AngleUnit, LengthUnit, ScaleUnit},
};

#[derive(Debug, PartialEq)]
pub struct MapProjection {
    pub map_projection_name: String,
    pub map_projection_method: MapProjectionMethod,
    pub map_projection_parameters: Option<Vec<MapProjectionParameter>>,
    pub identifier: Option<Id>, // TODO: Technically the spec allows for multiple
}

impl TryFrom<&WktNode> for MapProjection {
    type Error = WktParseError;

    fn try_from(value: &WktNode) -> Result<Self, Self::Error> {
        if value.keyword != CONVERSION {
            let expected = vec![CONVERSION.to_string()];
            return Err(WktParseError::IncorrectKeyword {
                expected: expected.into(),
                found: value.keyword.to_string(),
            });
        }

        if !(value.args.len() >= 2) {
            return Err(WktParseError::IncorrectArity {
                expected: vec!["2+".to_string()].into(),
                found: value.args.len(),
            });
        }

        let map_projection_name = match &value.args[0] {
            WktArg::String(s) => s.clone(),
            _ => return Err(WktParseError::ExpectedString),
        };

        let map_projection_method = match &value.args[1] {
            WktArg::Node(x) => MapProjectionMethod::try_from(x)?,
            _ => return Err(WktParseError::ExpectedNode),
        };

        let mut v = vec![];
        let mut identifier = None;

        for i in 2..value.args.len() {
            let this_value = &value.args[i];

            match this_value {
                WktArg::Node(node) => {
                    match node.keyword.as_str() {
                        PARAMETER => {
                            // Parameters must come before identifier

                            if identifier.is_some() {
                                return Err(WktParseError::IncorrectKeywordOrder);
                            }

                            let param = MapProjectionParameter::try_from(node)?;

                            v.push(param);
                        }
                        ID => {
                            let id = Id::try_from(node)?;
                            identifier = Some(id);
                        }
                        _ => {
                            return Err(WktParseError::IncorrectKeyword {
                                expected: vec![PARAMETER.to_string(), ID.to_string()].into(),
                                found: node.keyword.clone(),
                            });
                        }
                    }
                }
                _ => return Err(WktParseError::ExpectedNode),
            }
        }

        let map_projection_parameters = if v.is_empty() { None } else { Some(v) };

        Ok(MapProjection {
            map_projection_name,
            map_projection_method,
            map_projection_parameters,
            identifier,
        })
    }
}

#[derive(Debug, PartialEq)]
pub struct MapProjectionMethod {
    pub map_projection_method_name: String,
    pub identifier: Option<Id>, // TODO: Technically the spec allows for multiple
}

impl TryFrom<&WktNode> for MapProjectionMethod {
    type Error = WktParseError;

    fn try_from(value: &WktNode) -> Result<Self, Self::Error> {
        if !(value.keyword == METHOD || value.keyword == PROJECTION) {
            let expected = vec![METHOD.to_string(), PROJECTION.to_string()];
            return Err(WktParseError::IncorrectKeyword {
                expected: expected.into(),
                found: value.keyword.to_string(),
            });
        }

        if value.keyword == PROJECTION {
            warn!("Keyword SPHEROID depreciated. Consider using ELLIPSOID instead");
        }

        if !(value.args.len() == 1 || value.args.len() == 2) {
            return Err(WktParseError::IncorrectArity {
                expected: vec!["1".to_string(), "2".to_string()].into(),
                found: value.args.len(),
            });
        }

        let map_projection_method_name = match &value.args[0] {
            WktArg::String(s) => s.clone(),
            _ => return Err(WktParseError::ExpectedString),
        };

        let identifier = match value.args.get(1) {
            Some(x) => match x {
                WktArg::Node(node) => match Id::try_from(node) {
                    Ok(lu) => Some(lu),
                    Err(y) => return Err(y),
                },
                _ => return Err(WktParseError::ExpectedNode),
            },
            None => None,
        };

        Ok(MapProjectionMethod {
            map_projection_method_name,
            identifier,
        })
    }
}

#[derive(Debug, PartialEq)]
pub struct MapProjectionParameter {
    pub parameter_name: String,
    pub parameter_value: f64,
    pub map_projection_parameter_unit: Option<MapProjectionParameterUnit>,
    pub identifier: Option<Id>, // TODO: Technically the spec allows for multiple
}

impl TryFrom<&WktNode> for MapProjectionParameter {
    type Error = WktParseError;

    fn try_from(value: &WktNode) -> Result<Self, Self::Error> {
        if value.keyword != PARAMETER {
            let expected = vec![PARAMETER.to_string()];
            return Err(WktParseError::IncorrectKeyword {
                expected: expected.into(),
                found: value.keyword.to_string(),
            });
        }

        if !(value.args.len() == 2 || value.args.len() == 3 || value.args.len() == 4) {
            return Err(WktParseError::IncorrectArity {
                expected: vec![
                    "1".to_string(),
                    "2".to_string(),
                    "3".to_string(),
                    "4".to_string(),
                ]
                .into(),
                found: value.args.len(),
            });
        }

        let parameter_name = match &value.args[0] {
            WktArg::String(s) => s.clone(),
            _ => return Err(WktParseError::ExpectedString),
        };

        let parameter_value = match &value.args[1] {
            WktArg::Number(s) => s.clone(),
            _ => return Err(WktParseError::ExpectedNumber),
        };

        let map_projection_parameter_unit = match value.args.get(2) {
            Some(x) => match x {
                WktArg::Node(node) => match MapProjectionParameterUnit::try_from(node) {
                    Ok(pu) => Some(pu),
                    Err(y) => return Err(y),
                },
                _ => return Err(WktParseError::ExpectedNode),
            },
            None => None,
        };

        let identifier = match value.args.get(3) {
            Some(x) => match x {
                WktArg::Node(node) => match Id::try_from(node) {
                    Ok(lu) => Some(lu),
                    Err(y) => return Err(y),
                },
                _ => return Err(WktParseError::ExpectedNode),
            },
            None => None,
        };

        Ok(MapProjectionParameter {
            parameter_name,
            parameter_value,
            map_projection_parameter_unit,
            identifier,
        })
    }
}

#[derive(Debug, PartialEq)]
pub enum MapProjectionParameterUnit {
    LengthUnit(LengthUnit),
    AngleUnit(AngleUnit),
    ScaleUnit(ScaleUnit),
}

impl TryFrom<&WktNode> for MapProjectionParameterUnit {
    type Error = WktParseError;

    fn try_from(value: &WktNode) -> Result<Self, Self::Error> {
        match value.keyword.as_str() {
            LENGTHUNIT => {
                return match LengthUnit::try_from(value) {
                    Ok(x) => Ok(Self::LengthUnit(x)),
                    Err(y) => Err(y),
                };
            }
            ANGLEUNIT => {
                return match AngleUnit::try_from(value) {
                    Ok(x) => Ok(Self::AngleUnit(x)),
                    Err(y) => Err(y),
                };
            }
            SCALEUNIT => {
                return match ScaleUnit::try_from(value) {
                    Ok(x) => Ok(Self::ScaleUnit(x)),
                    Err(y) => Err(y),
                };
            }
            UNIT => {
                error!(
                    "Ambiguous Keyword `UNIT`. While older versions of the specification allowed for this keyword, in this context it is ambiguous and should be depreciated in favour of `LENGTHUNIT`, `ANGLEUNIT` or `SCALEUNIT`"
                );
                return Err(WktParseError::IncorrectKeyword {
                    expected: vec![
                        LENGTHUNIT.to_string(),
                        ANGLEUNIT.to_string(),
                        SCALEUNIT.to_string(),
                    ]
                    .into(),
                    found: UNIT.to_string(),
                });
            }
            _ => {
                return Err(WktParseError::IncorrectKeyword {
                    expected: vec![
                        LENGTHUNIT.to_string(),
                        ANGLEUNIT.to_string(),
                        SCALEUNIT.to_string(),
                    ]
                    .into(),
                    found: UNIT.to_string(),
                });
            }
        }
    }
}
