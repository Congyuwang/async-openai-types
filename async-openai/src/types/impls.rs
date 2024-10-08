use std::{
    fmt::Display,
    path::{Path, PathBuf},
};

use crate::types::InputSource;

use bytes::Bytes;

use super::{
    AudioInput, AudioResponseFormat, ChatCompletionFunctionCall, ChatCompletionNamedToolChoice,
    ChatCompletionRequestAssistantMessage, ChatCompletionRequestAssistantMessageContent,
    ChatCompletionRequestFunctionMessage, ChatCompletionRequestMessage,
    ChatCompletionRequestMessageContentPartImage, ChatCompletionRequestMessageContentPartText,
    ChatCompletionRequestSystemMessage, ChatCompletionRequestSystemMessageContent,
    ChatCompletionRequestToolMessage, ChatCompletionRequestToolMessageContent,
    ChatCompletionRequestUserMessage, ChatCompletionRequestUserMessageContent,
    ChatCompletionRequestUserMessageContentPart, ChatCompletionToolChoiceOption,
    CreateMessageRequestContent, DallE2ImageSize, EmbeddingInput, FileInput, FilePurpose,
    FunctionName, ImageInput, ImageModel, ImageResponseFormat, ImageSize, ImageUrl,
    ModerationInput, Prompt, Role, Stop, TimestampGranularity,
};

/// for `impl_from!(T, Enum)`, implements
/// - `From<T>`
/// - `From<Vec<T>>`
/// - `From<&Vec<T>>`
/// - `From<[T; N]>`
/// - `From<&[T; N]>`
///
/// for `T: Into<String>` and `Enum` having variants `String(String)` and `StringArray(Vec<String>)`
macro_rules! impl_from {
    ($from_typ:ty, $to_typ:ty) => {
        // From<T> -> String variant
        impl From<$from_typ> for $to_typ {
            fn from(value: $from_typ) -> Self {
                <$to_typ>::String(value.into())
            }
        }

        // From<Vec<T>> -> StringArray variant
        impl From<Vec<$from_typ>> for $to_typ {
            fn from(value: Vec<$from_typ>) -> Self {
                <$to_typ>::StringArray(value.iter().map(|v| v.to_string()).collect())
            }
        }

        // From<&Vec<T>> -> StringArray variant
        impl From<&Vec<$from_typ>> for $to_typ {
            fn from(value: &Vec<$from_typ>) -> Self {
                <$to_typ>::StringArray(value.iter().map(|v| v.to_string()).collect())
            }
        }

        // From<[T; N]> -> StringArray variant
        impl<const N: usize> From<[$from_typ; N]> for $to_typ {
            fn from(value: [$from_typ; N]) -> Self {
                <$to_typ>::StringArray(value.into_iter().map(|v| v.to_string()).collect())
            }
        }

        // From<&[T; N]> -> StringArray variatn
        impl<const N: usize> From<&[$from_typ; N]> for $to_typ {
            fn from(value: &[$from_typ; N]) -> Self {
                <$to_typ>::StringArray(value.into_iter().map(|v| v.to_string()).collect())
            }
        }
    };
}

// From String "family" to Prompt
impl_from!(&str, Prompt);
impl_from!(String, Prompt);
impl_from!(&String, Prompt);

// From String "family" to Stop
impl_from!(&str, Stop);
impl_from!(String, Stop);
impl_from!(&String, Stop);

// From String "family" to ModerationInput
impl_from!(&str, ModerationInput);
impl_from!(String, ModerationInput);
impl_from!(&String, ModerationInput);

// From String "family" to EmbeddingInput
impl_from!(&str, EmbeddingInput);
impl_from!(String, EmbeddingInput);
impl_from!(&String, EmbeddingInput);

/// for `impl_default!(Enum)`, implements `Default` for `Enum` as `Enum::String("")` where `Enum` has `String` variant
macro_rules! impl_default {
    ($for_typ:ty) => {
        impl Default for $for_typ {
            fn default() -> Self {
                Self::String("".into())
            }
        }
    };
}

impl_default!(Prompt);
impl_default!(ModerationInput);
impl_default!(EmbeddingInput);

impl Default for InputSource {
    fn default() -> Self {
        InputSource::Path {
            path: PathBuf::new(),
        }
    }
}

/// for `impl_input!(Struct)` where
/// ```text
/// Struct {
///     source: InputSource
/// }
/// ```
/// implements methods `from_bytes` and `from_vec_u8`,
/// and `From<P>` for `P: AsRef<Path>`
macro_rules! impl_input {
    ($for_typ:ty) => {
        impl $for_typ {
            pub fn from_bytes(filename: String, bytes: Bytes) -> Self {
                Self {
                    source: InputSource::Bytes { filename, bytes },
                }
            }

            pub fn from_vec_u8(filename: String, vec: Vec<u8>) -> Self {
                Self {
                    source: InputSource::VecU8 { filename, vec },
                }
            }
        }

        impl<P: AsRef<Path>> From<P> for $for_typ {
            fn from(path: P) -> Self {
                let path_buf = path.as_ref().to_path_buf();
                Self {
                    source: InputSource::Path { path: path_buf },
                }
            }
        }
    };
}

impl_input!(AudioInput);
impl_input!(FileInput);
impl_input!(ImageInput);

impl Display for ImageSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::S256x256 => "256x256",
                Self::S512x512 => "512x512",
                Self::S1024x1024 => "1024x1024",
                Self::S1792x1024 => "1792x1024",
                Self::S1024x1792 => "1024x1792",
            }
        )
    }
}

impl Display for DallE2ImageSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::S256x256 => "256x256",
                Self::S512x512 => "512x512",
                Self::S1024x1024 => "1024x1024",
            }
        )
    }
}

impl Display for ImageModel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::DallE2 => "dall-e-2",
                Self::DallE3 => "dall-e-3",
                Self::Other(other) => other,
            }
        )
    }
}

impl Display for ImageResponseFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Url => "url",
                Self::B64Json => "b64_json",
            }
        )
    }
}

impl Display for AudioResponseFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                AudioResponseFormat::Json => "json",
                AudioResponseFormat::Srt => "srt",
                AudioResponseFormat::Text => "text",
                AudioResponseFormat::VerboseJson => "verbose_json",
                AudioResponseFormat::Vtt => "vtt",
            }
        )
    }
}

impl Display for TimestampGranularity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                TimestampGranularity::Word => "word",
                TimestampGranularity::Segment => "segment",
            }
        )
    }
}

impl Display for Role {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Role::User => "user",
                Role::System => "system",
                Role::Assistant => "assistant",
                Role::Function => "function",
                Role::Tool => "tool",
            }
        )
    }
}

impl Display for FilePurpose {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Assistants => "assistants",
                Self::Batch => "batch",
                Self::FineTune => "fine-tune",
                Self::Vision => "vision",
            }
        )
    }
}

macro_rules! impl_from_for_integer_array {
    ($from_typ:ty, $to_typ:ty) => {
        impl<const N: usize> From<[$from_typ; N]> for $to_typ {
            fn from(value: [$from_typ; N]) -> Self {
                Self::IntegerArray(value.to_vec())
            }
        }

        impl<const N: usize> From<&[$from_typ; N]> for $to_typ {
            fn from(value: &[$from_typ; N]) -> Self {
                Self::IntegerArray(value.to_vec())
            }
        }

        impl From<Vec<$from_typ>> for $to_typ {
            fn from(value: Vec<$from_typ>) -> Self {
                Self::IntegerArray(value)
            }
        }

        impl From<&Vec<$from_typ>> for $to_typ {
            fn from(value: &Vec<$from_typ>) -> Self {
                Self::IntegerArray(value.clone())
            }
        }
    };
}

impl_from_for_integer_array!(u32, EmbeddingInput);
impl_from_for_integer_array!(u16, Prompt);

macro_rules! impl_from_for_array_of_integer_array {
    ($from_typ:ty, $to_typ:ty) => {
        impl From<Vec<Vec<$from_typ>>> for $to_typ {
            fn from(value: Vec<Vec<$from_typ>>) -> Self {
                Self::ArrayOfIntegerArray(value)
            }
        }

        impl From<&Vec<Vec<$from_typ>>> for $to_typ {
            fn from(value: &Vec<Vec<$from_typ>>) -> Self {
                Self::ArrayOfIntegerArray(value.clone())
            }
        }

        impl<const M: usize, const N: usize> From<[[$from_typ; N]; M]> for $to_typ {
            fn from(value: [[$from_typ; N]; M]) -> Self {
                Self::ArrayOfIntegerArray(value.iter().map(|inner| inner.to_vec()).collect())
            }
        }

        impl<const M: usize, const N: usize> From<[&[$from_typ; N]; M]> for $to_typ {
            fn from(value: [&[$from_typ; N]; M]) -> Self {
                Self::ArrayOfIntegerArray(value.iter().map(|inner| inner.to_vec()).collect())
            }
        }

        impl<const M: usize, const N: usize> From<&[[$from_typ; N]; M]> for $to_typ {
            fn from(value: &[[$from_typ; N]; M]) -> Self {
                Self::ArrayOfIntegerArray(value.iter().map(|inner| inner.to_vec()).collect())
            }
        }

        impl<const M: usize, const N: usize> From<&[&[$from_typ; N]; M]> for $to_typ {
            fn from(value: &[&[$from_typ; N]; M]) -> Self {
                Self::ArrayOfIntegerArray(value.iter().map(|inner| inner.to_vec()).collect())
            }
        }

        impl<const N: usize> From<[Vec<$from_typ>; N]> for $to_typ {
            fn from(value: [Vec<$from_typ>; N]) -> Self {
                Self::ArrayOfIntegerArray(value.to_vec())
            }
        }

        impl<const N: usize> From<&[Vec<$from_typ>; N]> for $to_typ {
            fn from(value: &[Vec<$from_typ>; N]) -> Self {
                Self::ArrayOfIntegerArray(value.to_vec())
            }
        }

        impl<const N: usize> From<[&Vec<$from_typ>; N]> for $to_typ {
            fn from(value: [&Vec<$from_typ>; N]) -> Self {
                Self::ArrayOfIntegerArray(value.into_iter().map(|inner| inner.clone()).collect())
            }
        }

        impl<const N: usize> From<&[&Vec<$from_typ>; N]> for $to_typ {
            fn from(value: &[&Vec<$from_typ>; N]) -> Self {
                Self::ArrayOfIntegerArray(
                    value
                        .to_vec()
                        .into_iter()
                        .map(|inner| inner.clone())
                        .collect(),
                )
            }
        }

        impl<const N: usize> From<Vec<[$from_typ; N]>> for $to_typ {
            fn from(value: Vec<[$from_typ; N]>) -> Self {
                Self::ArrayOfIntegerArray(value.into_iter().map(|inner| inner.to_vec()).collect())
            }
        }

        impl<const N: usize> From<&Vec<[$from_typ; N]>> for $to_typ {
            fn from(value: &Vec<[$from_typ; N]>) -> Self {
                Self::ArrayOfIntegerArray(value.into_iter().map(|inner| inner.to_vec()).collect())
            }
        }

        impl<const N: usize> From<Vec<&[$from_typ; N]>> for $to_typ {
            fn from(value: Vec<&[$from_typ; N]>) -> Self {
                Self::ArrayOfIntegerArray(value.into_iter().map(|inner| inner.to_vec()).collect())
            }
        }

        impl<const N: usize> From<&Vec<&[$from_typ; N]>> for $to_typ {
            fn from(value: &Vec<&[$from_typ; N]>) -> Self {
                Self::ArrayOfIntegerArray(value.into_iter().map(|inner| inner.to_vec()).collect())
            }
        }
    };
}

impl_from_for_array_of_integer_array!(u32, EmbeddingInput);
impl_from_for_array_of_integer_array!(u16, Prompt);

impl From<&str> for ChatCompletionFunctionCall {
    fn from(value: &str) -> Self {
        match value {
            "auto" => Self::Auto,
            "none" => Self::None,
            _ => Self::Function { name: value.into() },
        }
    }
}

impl From<&str> for FunctionName {
    fn from(value: &str) -> Self {
        Self { name: value.into() }
    }
}

impl From<String> for FunctionName {
    fn from(value: String) -> Self {
        Self { name: value }
    }
}

impl From<&str> for ChatCompletionNamedToolChoice {
    fn from(value: &str) -> Self {
        Self {
            r#type: super::ChatCompletionToolType::Function,
            function: value.into(),
        }
    }
}

impl From<String> for ChatCompletionNamedToolChoice {
    fn from(value: String) -> Self {
        Self {
            r#type: super::ChatCompletionToolType::Function,
            function: value.into(),
        }
    }
}

impl From<&str> for ChatCompletionToolChoiceOption {
    fn from(value: &str) -> Self {
        match value {
            "auto" => Self::Auto,
            "none" => Self::None,
            _ => Self::Named(value.into()),
        }
    }
}

impl From<String> for ChatCompletionToolChoiceOption {
    fn from(value: String) -> Self {
        match value.as_str() {
            "auto" => Self::Auto,
            "none" => Self::None,
            _ => Self::Named(value.into()),
        }
    }
}

// todo: write macro for bunch of same looking From trait implementations below

impl From<ChatCompletionRequestUserMessage> for ChatCompletionRequestMessage {
    fn from(value: ChatCompletionRequestUserMessage) -> Self {
        Self::User(value)
    }
}

impl From<ChatCompletionRequestSystemMessage> for ChatCompletionRequestMessage {
    fn from(value: ChatCompletionRequestSystemMessage) -> Self {
        Self::System(value)
    }
}

impl From<ChatCompletionRequestAssistantMessage> for ChatCompletionRequestMessage {
    fn from(value: ChatCompletionRequestAssistantMessage) -> Self {
        Self::Assistant(value)
    }
}

impl From<ChatCompletionRequestFunctionMessage> for ChatCompletionRequestMessage {
    fn from(value: ChatCompletionRequestFunctionMessage) -> Self {
        Self::Function(value)
    }
}

impl From<ChatCompletionRequestToolMessage> for ChatCompletionRequestMessage {
    fn from(value: ChatCompletionRequestToolMessage) -> Self {
        Self::Tool(value)
    }
}

impl From<ChatCompletionRequestUserMessageContent> for ChatCompletionRequestUserMessage {
    fn from(value: ChatCompletionRequestUserMessageContent) -> Self {
        Self {
            content: value,
            name: None,
        }
    }
}

impl From<ChatCompletionRequestSystemMessageContent> for ChatCompletionRequestSystemMessage {
    fn from(value: ChatCompletionRequestSystemMessageContent) -> Self {
        Self {
            content: value,
            name: None,
        }
    }
}

impl From<ChatCompletionRequestAssistantMessageContent> for ChatCompletionRequestAssistantMessage {
    fn from(value: ChatCompletionRequestAssistantMessageContent) -> Self {
        Self {
            content: Some(value),
            ..Default::default()
        }
    }
}

impl From<&str> for ChatCompletionRequestUserMessageContent {
    fn from(value: &str) -> Self {
        ChatCompletionRequestUserMessageContent::Text(value.into())
    }
}

impl From<String> for ChatCompletionRequestUserMessageContent {
    fn from(value: String) -> Self {
        ChatCompletionRequestUserMessageContent::Text(value)
    }
}

impl From<&str> for ChatCompletionRequestSystemMessageContent {
    fn from(value: &str) -> Self {
        ChatCompletionRequestSystemMessageContent::Text(value.into())
    }
}

impl From<String> for ChatCompletionRequestSystemMessageContent {
    fn from(value: String) -> Self {
        ChatCompletionRequestSystemMessageContent::Text(value)
    }
}

impl From<&str> for ChatCompletionRequestAssistantMessageContent {
    fn from(value: &str) -> Self {
        ChatCompletionRequestAssistantMessageContent::Text(value.into())
    }
}

impl From<String> for ChatCompletionRequestAssistantMessageContent {
    fn from(value: String) -> Self {
        ChatCompletionRequestAssistantMessageContent::Text(value)
    }
}

impl From<&str> for ChatCompletionRequestToolMessageContent {
    fn from(value: &str) -> Self {
        ChatCompletionRequestToolMessageContent::Text(value.into())
    }
}

impl From<String> for ChatCompletionRequestToolMessageContent {
    fn from(value: String) -> Self {
        ChatCompletionRequestToolMessageContent::Text(value)
    }
}

impl From<&str> for ChatCompletionRequestUserMessage {
    fn from(value: &str) -> Self {
        ChatCompletionRequestUserMessageContent::Text(value.into()).into()
    }
}

impl From<String> for ChatCompletionRequestUserMessage {
    fn from(value: String) -> Self {
        value.as_str().into()
    }
}

impl From<&str> for ChatCompletionRequestSystemMessage {
    fn from(value: &str) -> Self {
        ChatCompletionRequestSystemMessageContent::Text(value.into()).into()
    }
}

impl From<String> for ChatCompletionRequestSystemMessage {
    fn from(value: String) -> Self {
        value.as_str().into()
    }
}

impl From<&str> for ChatCompletionRequestAssistantMessage {
    fn from(value: &str) -> Self {
        ChatCompletionRequestAssistantMessageContent::Text(value.into()).into()
    }
}

impl From<String> for ChatCompletionRequestAssistantMessage {
    fn from(value: String) -> Self {
        value.as_str().into()
    }
}

impl From<Vec<ChatCompletionRequestUserMessageContentPart>>
    for ChatCompletionRequestUserMessageContent
{
    fn from(value: Vec<ChatCompletionRequestUserMessageContentPart>) -> Self {
        ChatCompletionRequestUserMessageContent::Array(value)
    }
}

impl From<ChatCompletionRequestMessageContentPartText>
    for ChatCompletionRequestUserMessageContentPart
{
    fn from(value: ChatCompletionRequestMessageContentPartText) -> Self {
        ChatCompletionRequestUserMessageContentPart::Text(value)
    }
}

impl From<ChatCompletionRequestMessageContentPartImage>
    for ChatCompletionRequestUserMessageContentPart
{
    fn from(value: ChatCompletionRequestMessageContentPartImage) -> Self {
        ChatCompletionRequestUserMessageContentPart::ImageUrl(value)
    }
}

impl From<&str> for ChatCompletionRequestMessageContentPartText {
    fn from(value: &str) -> Self {
        ChatCompletionRequestMessageContentPartText { text: value.into() }
    }
}

impl From<String> for ChatCompletionRequestMessageContentPartText {
    fn from(value: String) -> Self {
        ChatCompletionRequestMessageContentPartText { text: value }
    }
}

impl From<&str> for ImageUrl {
    fn from(value: &str) -> Self {
        Self {
            url: value.into(),
            detail: Default::default(),
        }
    }
}

impl From<String> for ImageUrl {
    fn from(value: String) -> Self {
        Self {
            url: value,
            detail: Default::default(),
        }
    }
}

impl From<String> for CreateMessageRequestContent {
    fn from(value: String) -> Self {
        Self::Content(value)
    }
}

impl From<&str> for CreateMessageRequestContent {
    fn from(value: &str) -> Self {
        Self::Content(value.to_string())
    }
}

impl Default for ChatCompletionRequestUserMessageContent {
    fn default() -> Self {
        ChatCompletionRequestUserMessageContent::Text("".into())
    }
}

impl Default for CreateMessageRequestContent {
    fn default() -> Self {
        Self::Content("".into())
    }
}

impl Default for ChatCompletionRequestSystemMessageContent {
    fn default() -> Self {
        ChatCompletionRequestSystemMessageContent::Text("".into())
    }
}

impl Default for ChatCompletionRequestToolMessageContent {
    fn default() -> Self {
        ChatCompletionRequestToolMessageContent::Text("".into())
    }
}
