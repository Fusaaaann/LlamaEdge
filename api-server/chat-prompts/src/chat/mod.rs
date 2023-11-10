pub mod baichuan;
pub mod belle;
pub mod chatml;
pub mod llama;
pub mod mistral;
pub mod openchat;
pub mod vicuna;

use crate::error::Result;
use baichuan::*;
use belle::*;
use chatml::*;
use endpoints::chat::ChatCompletionRequestMessage;
use llama::*;
use mistral::*;
use openchat::*;
use vicuna::*;

#[enum_dispatch::enum_dispatch]
pub trait BuildChatPrompt: Send {
    fn build(&self, messages: &mut Vec<ChatCompletionRequestMessage>) -> Result<String>;
}

#[enum_dispatch::enum_dispatch(BuildChatPrompt)]
pub enum ChatPrompt {
    Llama2ChatPrompt,
    MistralInstructPrompt,
    MistralLitePrompt,
    OpenChatPrompt,
    CodeLlamaInstructPrompt,
    BelleLlama2ChatPrompt,
    VicunaChatPrompt,
    ChatMLPrompt,
    Baichuan2ChatPrompt,
}
