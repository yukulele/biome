use crate::{
    Serializable, SerializableValue, SerializationDiagnostic, SerializationVisitor, VisitableType,
};
use biome_json_factory::make;
use biome_json_factory::make::json_boolean_value;
use biome_json_syntax::{JsonBooleanValue, TextRange, T};

impl Serializable for JsonBooleanValue {
    fn serialize(
        value: &impl SerializableValue,
        name: &str,
        diagnostics: &mut Vec<SerializationDiagnostic>,
    ) -> Option<Self> {
        struct Visitor;
        impl SerializationVisitor for Visitor {
            type Output = JsonBooleanValue;
            const EXPECTED_TYPE: VisitableType = VisitableType::BOOL;
            fn visit_bool(
                self,
                value: bool,
                _range: TextRange,
                _name: &str,
                _diagnostics: &mut Vec<SerializationDiagnostic>,
            ) -> Option<Self::Output> {
                let token = if value { T![true] } else { T![false] };
                Some(json_boolean_value(make::token(token)))
            }
        }
        value.serialize(Visitor, name, diagnostics)
    }
}
