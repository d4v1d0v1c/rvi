use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, Read};
use std::path::{Path, PathBuf};

use crate::error::*;
use content_inspector::{self, ContentType};
use clircle::{Clircle, Identifier};

#[derive(Clone, Default)]
pub(crate) struct InputMetadata {
    pub(crate) user_provided_name: Option<PathBuf>,
    pub(crate) size: Option<u64>,
}

#[derive(Clone)]
pub struct InputDescription {
    pub(crate) name: String,
    title : Option<String>,
    kind : Option<String>,
    summary : Option<String>,
}

impl InputDescription {
    pub fn new(name: impl Into<String>) -> Self {
        InputDescription {
            name : name.into(),
            title : None,
            kind : None,
            summary : None,
        }
    }
    pub fn set_kind(&mut self, kind: Option<String>) {
        self.kind = kind;
    }

    pub fn set_summary(&mut self, summary : Option<String>) {
        self.summary = summary;
    }
    
    pub fn set_title(&mut self, title : Option<String>) {
        self.title = title;
    }

    pub fn title(&self) -> &String {
        match &self.title {
            Some(title) => title,
            None => &self.name,
        }        
    }

    pub fn kind(&self) -> Option<&String> {
        self.kind.as_ref()
    }

    pub fn summary(&self) -> String {        
        self.summary.clone().unwrap_or_else(|| match &self.kind {
            None => self.name.clone(),
            Some(kind) => format!("{} '{}'", kind.to_lowercase(), self.name),
        })
    }
}

pub(crate) enum InputKind<'a> {
    OrdinaryFile (PathBuf),
    StdIn,
    CustomReader(Box<dyn Read + 'a>),
}

impl InputKind<'_> {
    pub fn description(&self) -> InputDescription {
        match self {
            InputKind::OrdinaryFile(path) => InputDescription::new(path.to_string_lossy()),
            InputKind::StdIn => InputDescription::new("STDIN"),
            InputKind::CustomReader(_) => InputDescription::new("READER"),
        }
    }    
}

pub struct Input<'a> {
    pub(crate) kind : InputKind<'a>,    
    pub(crate) metadata: InputMetadata,
    pub(crate) description: InputDescription,
}

impl<'a> Input<'a> {
    pub fn ordinary_file(path: impl AsRef<Path>) -> Self {
        Self::_ordinary_file(path.as_ref())
    }

    pub fn _ordinary_file(path: &Path) -> Self {
        let kind = InputKind::OrdinaryFile(path.to_path_buf());
        let metadata = InputMetadata {
            size : fs::metadata(path).map(|m| m.len()).ok(),
            ..InputMetadata::default()
        };
        Input  {
            description : kind.description(),
            metadata,
            kind,        
        }
    }

    pub fn stdin() -> Self {
        let kind = InputKind::StdIn;
        Input {
            description: kind.description(),
            metadata: InputMetadata::default(),
            kind,
        }
    }

    pub fn from_reader(reader : Box<dyn Read + 'a>) -> Self {
        let kind = InputKind::CustomReader(reader);
        Input {
            description : kind.description(),
            metadata : InputMetadata::default(),
            kind
        }     
    }    

    pub fn is_stdin(&self) -> bool {
        matches!(self.kind, InputKind::StdIn)
    }

    pub fn with_name(self, provided_name : Option<impl AsRef<Path>>) -> Self {
        self._with_name(provided_name.as_ref().map(|it| it.as_ref()))
    }

    pub fn _with_name(mut self, provided_name : Option<&Path>) -> Self {
        if let Some(name) = provided_name {
            self.description.name = name.to_string_lossy().to_string();
        }
        self.metadata.user_provided_name = provided_name.map(|n| n.to_owned());
        self
    }

    pub fn description(&self) -> &InputDescription {
        &self.description
    }

    pub fn description_mut(&mut self) -> &mut InputDescription {
        &mut self.description
    }

    pub(crate) fn open<R: BufRead + 'a>(
        self,
        stdin : R,
        stdout_identifier: Option<&Identifier>) -> Result<OpenedInput<'a>> {
            let description = self.description().clone();
                    match self.kind {
            InputKind::StdIn => {
                if let Some(stdout) = stdout_identifier {
                    let input_identifier = Identifier::try_from(clircle::Stdio::Stdin)
                        .map_err(|e| format!("Stdin: Error identifying file: {e}"))?;
                    if stdout.surely_conflicts_with(&input_identifier) {
                        return Err("IO circle detected. The input from stdin is also an output. Aborting to avoid infinite loop.".into());
                    }
                }

                Ok(OpenedInput {
                    kind: OpenedInputKind::StdIn,
                    description,
                    metadata: self.metadata,
                    reader: InputReader::new(stdin),
                })
            }

            InputKind::OrdinaryFile(path) => Ok(OpenedInput {
                kind: OpenedInputKind::OrdinaryFile(path.clone()),
                description,
                metadata: self.metadata,
                reader: {
                    let mut file = File::open(&path)
                        .map_err(|e| format!("'{}': {}", path.to_string_lossy(), e))?;
                    if file.metadata()?.is_dir() {
                        return Err(format!("'{}' is a directory.", path.to_string_lossy()).into());
                    }

                    if let Some(stdout) = stdout_identifier {
                        let input_identifier = Identifier::try_from(file).map_err(|e| {
                            format!("{}: Error identifying file: {}", path.to_string_lossy(), e)
                        })?;
                        if stdout.surely_conflicts_with(&input_identifier) {
                            return Err(format!(
                                "IO circle detected. The input from '{}' is also an output. Aborting to avoid infinite loop.",
                                path.to_string_lossy()
                            )
                            .into());
                        }
                        file = input_identifier.into_inner().expect("The file was lost in the clircle::Identifier, this should not have happened...");
                    }

                    InputReader::new(BufReader::new(file))
                },
            }),
            InputKind::CustomReader(reader) => Ok(OpenedInput {
                description,
                kind: OpenedInputKind::CustomReader,
                metadata: self.metadata,
                reader: InputReader::new(BufReader::new(reader)),
            }),
        }
    }
}

pub(crate) struct InputReader<'a> {
    inner: Box<dyn BufRead + 'a>,
    pub(crate) first_line: Vec<u8>,
    pub(crate) content_type: Option<ContentType>,
}

impl<'a> InputReader<'a> {
        pub(crate) fn new<R: BufRead + 'a>(mut reader: R) -> InputReader<'a> {
        let mut first_line = vec![];
        reader.read_until(b'\n', &mut first_line).ok();

        let content_type = if first_line.is_empty() {
            None
        } else {
            Some(content_inspector::inspect(&first_line[..]))
        };

        if content_type == Some(ContentType::UTF_16LE) {
            reader.read_until(0x00, &mut first_line).ok();
        }

        InputReader {
            inner: Box::new(reader),
            first_line,
            content_type,
        }
    }

    pub(crate) fn read_line(&mut self, buf: &mut Vec<u8>) -> io::Result<bool> {
        if !self.first_line.is_empty() {
            buf.append(&mut self.first_line);
            return Ok(true);
        }

        let res = self.inner.read_until(b'\n', buf).map(|size| size > 0)?;

        if self.content_type == Some(ContentType::UTF_16LE) {
            let _ = self.inner.read_until(0x00, buf);
        }

        Ok(res)
    }
}
pub(crate) enum OpenedInputKind {
    OrdinaryFile(PathBuf),
    StdIn,
    CustomReader,
}
pub struct OpenedInput<'a> {
    pub(crate) kind : OpenedInputKind,
    pub(crate) metadata : InputMetadata,
    pub(crate) reader : InputReader<'a>,
    pub(crate) description : InputDescription,
}

impl OpenedInput<'_> {
    pub(crate) fn path(&self) -> Option<&PathBuf> {
        self.metadata.user_provided_name.as_ref().or(match self.kind {
            OpenedInputKind::OrdinaryFile(ref path) => Some(path),
            _ => None,
        })
    }
}

