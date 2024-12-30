use std::path::PathBuf;

use docx_rust::{
    document::{Paragraph, Run, TextSpace},
    formatting::{CharacterProperty, JustificationVal, ParagraphProperty},
    Docx,
};

use crate::parse::{Content, Document, Style};

pub(crate) fn docx(input: Document, mut path: PathBuf) {
    let mut doc = Docx::default();
    for content in input.into_iter() {
        let mut para = Paragraph::default();
        match content {
            Content::Title(text) => {
                let prop = ParagraphProperty::default().justification(JustificationVal::Center);
                para = para.property(prop);
                for t in text.into_iter() {
                    para = para.push_text(t.text);
                }
            }
            Content::Heading { text, level } => {
                let lvls: [isize; 6] = [36, 24, 20, 18, 16, 14];
                let size = lvls[usize::min(level, lvls.len() - 1)];
                para = para.push(
                    Run::default()
                        .property(CharacterProperty::default().bold(true).size(size))
                        .push_text(
                            text.into_iter()
                                .map(|t| t.text)
                                .collect::<Vec<String>>()
                                .join(""),
                        ),
                );
            }
            Content::Paragraph(text) => {
                for t in text {
                    let mut prop = CharacterProperty::default()
                        .bold(t.styles.contains(&Style::Bold))
                        .italics(t.styles.contains(&Style::Italics))
                        .strike(t.styles.contains(&Style::Strikethrough));
                    if t.styles.contains(&Style::Underline) {
                        prop = prop.underline("000000");
                    }
                    dbg!(&t);
                    para = para.push(
                        Run::default()
                            .property(prop)
                            .push_text((t.text, TextSpace::Preserve)),
                    );
                }
            }
            _ => (),
        }
        doc.document.push(para);
    }

    path.set_extension("docx");
    doc.write_file(path).unwrap();
}
