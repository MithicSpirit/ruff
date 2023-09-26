use std::io::Write;

use serde::ser::SerializeSeq;
use serde::{Serialize, Serializer};
use serde_json::{json, Value};

use ruff_diagnostics::Edit;
use ruff_notebook::NotebookIndex;
use ruff_source_file::SourceCode;
use ruff_text_size::Ranged;

use crate::message::{Emitter, EmitterContext, Message};
use crate::registry::AsRule;

#[derive(Default)]
pub struct JsonEmitter;

impl Emitter for JsonEmitter {
    fn emit(
        &mut self,
        writer: &mut dyn Write,
        messages: &[Message],
        context: &EmitterContext,
    ) -> anyhow::Result<()> {
        serde_json::to_writer_pretty(writer, &ExpandedMessages { messages, context })?;

        Ok(())
    }
}

struct ExpandedMessages<'a> {
    messages: &'a [Message],
    context: &'a EmitterContext<'a>,
}

impl Serialize for ExpandedMessages<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_seq(Some(self.messages.len()))?;

        for message in self.messages {
            let value = message_to_json_value(message, self.context);
            s.serialize_element(&value)?;
        }

        s.end()
    }
}

pub(crate) fn message_to_json_value(message: &Message, context: &EmitterContext) -> Value {
    let source_code = message.file.to_source_code();
    let notebook_index = context.notebook_index(message.filename());

    let fix = message.fix.as_ref().map(|fix| {
        json!({
            "applicability": fix.applicability(),
            "message": message.kind.suggestion.as_deref(),
            "edits": &ExpandedEdits { edits: fix.edits(), source_code: &source_code, notebook_index },
        })
    });

    let mut start_location = source_code.source_location(message.start());
    let mut end_location = source_code.source_location(message.end());
    let mut noqa_location = source_code.source_location(message.noqa_offset);
    let mut notebook_cell_index = None;

    if let Some(notebook_index) = notebook_index {
        notebook_cell_index = Some(notebook_index.cell(start_location.row.get()).unwrap_or(1));
        start_location = notebook_index.translated_location(&start_location);
        end_location = notebook_index.translated_location(&end_location);
        noqa_location = notebook_index.translated_location(&noqa_location);
    }

    json!({
        "code": message.kind.rule().noqa_code().to_string(),
        "url": message.kind.rule().url(),
        "message": message.kind.body,
        "fix": fix,
        "cell": notebook_cell_index,
        "location": start_location,
        "end_location": end_location,
        "filename": message.filename(),
        "noqa_row": noqa_location.row
    })
}

struct ExpandedEdits<'a> {
    edits: &'a [Edit],
    source_code: &'a SourceCode<'a, 'a>,
    notebook_index: Option<&'a NotebookIndex>,
}

impl Serialize for ExpandedEdits<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_seq(Some(self.edits.len()))?;

        for edit in self.edits {
            let mut location = self.source_code.source_location(edit.start());
            let mut end_location = self.source_code.source_location(edit.end());

            if let Some(notebook_index) = self.notebook_index {
                location = notebook_index.translated_location(&location);
                end_location = notebook_index.translated_location(&end_location);
            }

            let value = json!({
                "content": edit.content().unwrap_or_default(),
                "location": location,
                "end_location": end_location
            });

            s.serialize_element(&value)?;
        }

        s.end()
    }
}

#[cfg(test)]
mod tests {
    use insta::assert_snapshot;

    use crate::message::tests::{capture_emitter_output, create_messages};
    use crate::message::JsonEmitter;

    #[test]
    fn output() {
        let mut emitter = JsonEmitter;
        let content = capture_emitter_output(&mut emitter, &create_messages());

        assert_snapshot!(content);
    }
}
