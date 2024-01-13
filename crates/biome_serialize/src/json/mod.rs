mod impls;

#[cfg(test)]
mod tests {
    use biome_json_factory::make;
    use biome_json_factory::make::{json_member_list, json_member_name, json_object_value};
    use biome_json_syntax::{JsonObjectValue, T, TextRange};
    use biome_rowan::BatchMutationExt;
    use crate::{Serializable, SerializableValue, SerializationDiagnostic, SerializationVisitor};

    #[test]
    fn struct_ok() {

        struct Test { value: bool };

        struct TestVisitor;

        impl SerializationVisitor for Test {

            type Output = JsonObjectValue;
            fn visit_struct<S>(self, value: S, range: TextRange, name: &str, diagnostics: &mut Vec<SerializationDiagnostic>) -> Option<Self::Output> {
                let mut object = json_object_value(make::token(T!['{']), make::token(T!['}']));
                let name = json_member_name();
                let value = Serializable::serialize(value)
                let list= json_member_list([], []);
                let object = object.with_json_member_list(list).build();
                Some(object)
            }
        }

        impl Serializable for Test {
            fn serialize( value: &impl SerializableValue, name: &str, diagnostics: &mut Vec<SerializationDiagnostic>) -> Option<Self> {
                value.serialize(self, name, diagnostics)

            }
        }

    }
}
