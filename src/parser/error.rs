use std::fmt;

use ariadne::{ Color, Config, Label, Report, ReportKind, Source };

use crate::lexer::Span;

pub struct ParseError {
    pub message: String,
    pub span: Option<Span>,
}

impl ParseError {
    pub fn new<S: Into<String>>(message: S, span: Option<Span>) -> Self {
        Self {
            message: message.into(),
            span,
        }
    }

    pub fn display(&self, source_id: &str, source_code: &str) {
        let mut report = Report::build(ReportKind::Error, (source_id, 0..0)).with_message(
            &self.message
        );

        if let Some(span) = &self.span {
            report = report.with_config(Config::default().with_compact(false)).with_label(
                Label::new((source_id, span.start..span.end))
                    .with_color(Color::Red)
                    .with_message(&self.message)
                    .with_order(1)
            );
        }

        report
            .finish()
            .print((source_id, Source::from(source_code)))
            .unwrap();
    }
}

impl fmt::Debug for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.span {
            Some(span) => write!(f, "[{}..{}] {}", span.start, span.end, self.message),
            None => write!(f, "{}", self.message),
        }
    }
}
