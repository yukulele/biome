use crate::VisitableType;
use biome_diagnostics::console::fmt::Display;
use biome_diagnostics::console::{markup, MarkupBuf};
use biome_diagnostics::location::AsSpan;
use biome_diagnostics::{
    Advices, Diagnostic, DiagnosticTags, LogCategory, MessageAndDescription, Severity, Visit,
};
use biome_json_syntax::TextRange;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Clone, Deserialize, Diagnostic)]
pub struct SerializationDiagnostic {
    #[message]
    #[description]
    reason: MessageAndDescription,
    #[location(span)]
    range: Option<TextRange>,
    #[advice]
    deserialization_advice: SerializationAdvice,
    #[severity]
    severity: Severity,
    #[tags]
    tags: DiagnosticTags,
}

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct SerializationAdvice {
    notes: Vec<(MarkupBuf, Vec<MarkupBuf>)>,
}

impl SerializationAdvice {
    pub fn note(mut self, message: impl Display) -> Self {
        self.notes
            .push((markup! {{message}}.to_owned(), Vec::new()));
        self
    }
}

impl Advices for SerializationAdvice {
    fn record(&self, visitor: &mut dyn Visit) -> std::io::Result<()> {
        for (message, known_keys) in &self.notes {
            visitor.record_log(LogCategory::Info, message)?;
            if !known_keys.is_empty() {
                let list: Vec<_> = known_keys
                    .iter()
                    .map(|message| message as &dyn Display)
                    .collect();
                visitor.record_list(&list)?;
            }
        }

        Ok(())
    }
}

impl SerializationDiagnostic {
    pub fn new(reason: impl Display) -> Self {
        Self {
            reason: markup! {{reason}}.to_owned().into(),
            range: None,
            deserialization_advice: SerializationAdvice::default(),
            severity: Severity::Error,
            tags: DiagnosticTags::empty(),
        }
    }

    /// Emitted when a generic node has an incorrect type
    pub fn new_incorrect_type(
        actual_type: VisitableType,
        expected_type: VisitableType,
        range: impl AsSpan,
    ) -> Self {
        Self::new(markup! {
            "Incorrect type, expected "<Emphasis>{format_args!("{}", expected_type)}</Emphasis>", but received "<Emphasis>{format_args!("{}", actual_type)}</Emphasis>"."
        })
            .with_range(range)
    }

    /// Emitted when a generic node has an incorrect type
    pub fn new_incorrect_type_with_name(
        actual_type: VisitableType,
        expected_type: VisitableType,
        name: &str,
        range: impl AsSpan,
    ) -> Self {
        if name.is_empty() {
            return Self::new_incorrect_type(actual_type, expected_type, range);
        }
        Self::new(markup! {
            <Emphasis>{name}</Emphasis>" has an incorrect type, expected "<Emphasis>{format_args!("{}", expected_type)}</Emphasis>", but received "<Emphasis>{format_args!("{}", actual_type)}</Emphasis>"."
        })
            .with_range(range)
    }

    /// Adds a range to the diagnostic
    pub fn with_range(mut self, span: impl AsSpan) -> Self {
        self.range = span.as_span();
        self
    }

    /// Adds a note to the diagnostic
    pub fn with_note(mut self, message: impl Display) -> Self {
        self.deserialization_advice
            .notes
            .push((markup! {{message}}.to_owned(), vec![]));
        self
    }
}
