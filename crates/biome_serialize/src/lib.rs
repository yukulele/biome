pub use crate::diagnostics::SerializationDiagnostic;
use biome_json_syntax::TextRange;
use biome_rowan::AstNode;
use bitflags::bitflags;

mod diagnostics;
mod json;

pub trait Serializable: Sized {
    fn serialize(
        value: &impl SerializableValue,
        name: &str,
        diagnostics: &mut Vec<SerializationDiagnostic>,
    ) -> Option<Self>;
}

/// Implemented by data structure that can be deserialized.
///
/// This trait should only be implemented when adding the support for a new data format.
/// See [biome_deserialize::json] for an example of an implementation.
pub trait SerializableValue: Sized {
    /// Range in the source content of this value
    fn range(&self) -> TextRange;

    /// Returns the deserialized form of this value using `visitor`.
    /// Any diagnostics emitted during deserialization are appended to `diagnostics`.
    /// `name` corresponds to the name used in a diagnostic to designate the value.
    fn serialize<V: SerializationVisitor>(
        &self,
        visitor: V,
        name: &str,
        diagnostics: &mut Vec<SerializationDiagnostic>,
    ) -> Option<V::Output>;
}

pub trait SerializationVisitor: Sized {
    /// The type of the deserialized form of the visited value.
    type Output: AstNode;

    /// The expected type of the visited value.
    const EXPECTED_TYPE: VisitableType;

    fn visit_bool(
        self,
        _value: bool,
        range: TextRange,
        name: &str,
        diagnostics: &mut Vec<SerializationDiagnostic>,
    ) -> Option<Self::Output> {
        debug_assert!(
            !Self::EXPECTED_TYPE.contains(VisitableType::NULL),
            "This method should be implemented because the expected type is null."
        );
        diagnostics.push(SerializationDiagnostic::new_incorrect_type_with_name(
            VisitableType::NULL,
            Self::EXPECTED_TYPE,
            name,
            range,
        ));
        None
    }

    fn visit_struct<S>(
        self,
        _value: S,
        range: TextRange,
        name: &str,
        diagnostics: &mut Vec<SerializationDiagnostic>,
    ) -> Option<Self::Output> {
        debug_assert!(
            !Self::EXPECTED_TYPE.contains(VisitableType::STRUCT),
            "This method should be implemented because the expected type is struct."
        );
        diagnostics.push(SerializationDiagnostic::new_incorrect_type_with_name(
            VisitableType::STRUCT,
            Self::EXPECTED_TYPE,
            name,
            range,
        ));
        None
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq)]
    pub struct VisitableType: u8 {
        const NULL = 1 << 0;
        const BOOL = 1 << 1;
        const NUMBER = 1 << 2;
        const STR = 1 << 3;
        const STRUCT = 1 << 4;
        const MAP = 1 << 5;
    }
}

impl std::fmt::Display for VisitableType {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.is_empty() {
            return write!(fmt, "no value");
        }
        for (i, expected_type) in self.iter().enumerate() {
            if i != 0 {
                write!(fmt, ", or ")?;
            }
            let expected_type = match expected_type {
                VisitableType::NULL => "null",
                VisitableType::BOOL => "a boolean",
                VisitableType::NUMBER => "a number",
                VisitableType::STR => "a string",
                VisitableType::STRUCT => "an struct",
                VisitableType::MAP => "an object",
                _ => unreachable!("Unhandled deserialization type."),
            };
            write!(fmt, "{}", expected_type)?;
        }
        Ok(())
    }
}
