use serde::{Deserialize, Serialize};

use crate::protocol::ApiProtocol;

/// Supported LLM API providers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub enum ApiProvider {
    // Primary
    Claude,
    OpenAI,
    Gemini,
    // Extended ecosystem
    DeepSeek,
    Groq,
    Mistral,
    XAI,
    Together,
    Fireworks,
    Perplexity,
    Cohere,
    OpenRouter,
    // China region
    Moonshot,
    GLM,
    Qwen,
    Doubao,
    MiniMax,
    Hunyuan,
    // Local / self-hosted
    Ollama,
    LMStudio,
    LlamaCPP,
    VLLM,
}

/// Static metadata for each provider.
pub struct ProviderMeta {
    pub provider: ApiProvider,
    pub name: &'static str,
    pub display_name: &'static str,
    pub base_url: &'static str,
    pub env_vars: &'static [&'static str],
    pub default_model: &'static str,
    pub protocol: ApiProtocol,
    pub local: bool,
}

/// Full provider registry.
pub static PROVIDERS: &[ProviderMeta] = &[
    // ── Primary ──────────────────────────────────────────
    ProviderMeta { provider: ApiProvider::Claude,     name: "claude",     display_name: "Claude",      base_url: "https://api.anthropic.com/v1/messages",                    env_vars: &["ANTHROPIC_API_KEY"],               default_model: "claude-sonnet-4-20250514", protocol: ApiProtocol::Anthropic,    local: false },
    ProviderMeta { provider: ApiProvider::OpenAI,     name: "openai",     display_name: "OpenAI",      base_url: "https://api.openai.com/v1",                                env_vars: &["OPENAI_API_KEY"],                  default_model: "gpt-4o",                   protocol: ApiProtocol::OpenAiCompat,  local: false },
    ProviderMeta { provider: ApiProvider::Gemini,     name: "gemini",     display_name: "Gemini",      base_url: "https://generativelanguage.googleapis.com/v1beta",         env_vars: &["GEMINI_API_KEY"],                  default_model: "gemini-2.0-flash",         protocol: ApiProtocol::Gemini,        local: false },
    // ── Extended ecosystem ───────────────────────────────
    ProviderMeta { provider: ApiProvider::DeepSeek,   name: "deepseek",   display_name: "DeepSeek",    base_url: "https://api.deepseek.com",                                 env_vars: &["DEEPSEEK_API_KEY"],                default_model: "deepseek-chat",            protocol: ApiProtocol::OpenAiCompat,  local: false },
    ProviderMeta { provider: ApiProvider::Groq,       name: "groq",       display_name: "Groq",        base_url: "https://api.groq.com/openai/v1",                           env_vars: &["GROQ_API_KEY"],                    default_model: "llama-3.3-70b-versatile",  protocol: ApiProtocol::OpenAiCompat,  local: false },
    ProviderMeta { provider: ApiProvider::Mistral,    name: "mistral",    display_name: "Mistral",     base_url: "https://api.mistral.ai/v1",                                env_vars: &["MISTRAL_API_KEY"],                 default_model: "mistral-large-latest",     protocol: ApiProtocol::OpenAiCompat,  local: false },
    ProviderMeta { provider: ApiProvider::XAI,        name: "xai",        display_name: "xAI (Grok)",  base_url: "https://api.x.ai",                                         env_vars: &["XAI_API_KEY"],                     default_model: "grok-3",                   protocol: ApiProtocol::OpenAiCompat,  local: false },
    ProviderMeta { provider: ApiProvider::Together,   name: "together",   display_name: "Together AI", base_url: "https://api.together.xyz",                                  env_vars: &["TOGETHER_API_KEY"],                default_model: "meta-llama/Llama-3-70b-chat-hf", protocol: ApiProtocol::OpenAiCompat, local: false },
    ProviderMeta { provider: ApiProvider::Fireworks,  name: "fireworks",  display_name: "Fireworks",   base_url: "https://api.fireworks.ai/inference/v1",                     env_vars: &["FIREWORKS_API_KEY"],               default_model: "accounts/fireworks/models/llama-v3p1-70b-instruct", protocol: ApiProtocol::OpenAiCompat, local: false },
    ProviderMeta { provider: ApiProvider::Perplexity, name: "perplexity", display_name: "Perplexity",  base_url: "https://api.perplexity.ai",                                env_vars: &["PERPLEXITY_API_KEY"],              default_model: "sonar-pro",                protocol: ApiProtocol::OpenAiCompat,  local: false },
    ProviderMeta { provider: ApiProvider::Cohere,     name: "cohere",     display_name: "Cohere",      base_url: "https://api.cohere.com/compatibility",                      env_vars: &["COHERE_API_KEY"],                  default_model: "command-r-plus",           protocol: ApiProtocol::OpenAiCompat,  local: false },
    ProviderMeta { provider: ApiProvider::OpenRouter,  name: "openrouter", display_name: "OpenRouter",  base_url: "https://openrouter.ai/api/v1",                             env_vars: &["OPENROUTER_API_KEY"],              default_model: "anthropic/claude-sonnet-4", protocol: ApiProtocol::OpenAiCompat, local: false },
    // ── China region ─────────────────────────────────────
    ProviderMeta { provider: ApiProvider::Moonshot,   name: "moonshot",   display_name: "Moonshot (Kimi)", base_url: "https://api.moonshot.cn/v1",                            env_vars: &["MOONSHOT_API_KEY"],                default_model: "moonshot-v1-8k",           protocol: ApiProtocol::OpenAiCompat,  local: false },
    ProviderMeta { provider: ApiProvider::GLM,        name: "glm",        display_name: "GLM (智谱)",    base_url: "https://open.bigmodel.cn/api/paas/v4",                     env_vars: &["GLM_API_KEY"],                     default_model: "glm-4-flash",              protocol: ApiProtocol::OpenAiCompat,  local: false },
    ProviderMeta { provider: ApiProvider::Qwen,       name: "qwen",       display_name: "Qwen (通义)",   base_url: "https://dashscope.aliyuncs.com/compatible-mode/v1",        env_vars: &["DASHSCOPE_API_KEY"],               default_model: "qwen-plus",                protocol: ApiProtocol::OpenAiCompat,  local: false },
    ProviderMeta { provider: ApiProvider::Doubao,     name: "doubao",     display_name: "Doubao (豆包)", base_url: "https://ark.cn-beijing.volces.com/api/v3",                 env_vars: &["ARK_API_KEY", "DOUBAO_API_KEY"],   default_model: "doubao-1.5-pro-32k",       protocol: ApiProtocol::OpenAiCompat,  local: false },
    ProviderMeta { provider: ApiProvider::MiniMax,    name: "minimax",    display_name: "MiniMax",     base_url: "https://api.minimaxi.com/v1",                               env_vars: &["MINIMAX_API_KEY"],                 default_model: "MiniMax-Text-01",          protocol: ApiProtocol::OpenAiCompat,  local: false },
    ProviderMeta { provider: ApiProvider::Hunyuan,    name: "hunyuan",    display_name: "Hunyuan (混元)", base_url: "https://api.hunyuan.cloud.tencent.com/v1",                env_vars: &["HUNYUAN_API_KEY"],                 default_model: "hunyuan-turbos-latest",    protocol: ApiProtocol::OpenAiCompat,  local: false },
    // ── Local / self-hosted ──────────────────────────────
    ProviderMeta { provider: ApiProvider::Ollama,     name: "ollama",     display_name: "Ollama",      base_url: "http://localhost:11434/v1",                                 env_vars: &["OLLAMA_API_KEY"],                  default_model: "llama3",                   protocol: ApiProtocol::OpenAiCompat,  local: true },
    ProviderMeta { provider: ApiProvider::LMStudio,   name: "lmstudio",   display_name: "LM Studio",   base_url: "http://localhost:1234/v1",                                  env_vars: &[],                                  default_model: "local-model",              protocol: ApiProtocol::OpenAiCompat,  local: true },
    ProviderMeta { provider: ApiProvider::LlamaCPP,   name: "llamacpp",   display_name: "llama.cpp",   base_url: "http://localhost:8080/v1",                                  env_vars: &["LLAMACPP_API_KEY"],                default_model: "local-model",              protocol: ApiProtocol::OpenAiCompat,  local: true },
    ProviderMeta { provider: ApiProvider::VLLM,       name: "vllm",       display_name: "vLLM",        base_url: "http://localhost:8000/v1",                                  env_vars: &["VLLM_API_KEY"],                    default_model: "local-model",              protocol: ApiProtocol::OpenAiCompat,  local: true },
];

/// Look up provider metadata by enum variant.
pub fn provider_meta(provider: ApiProvider) -> &'static ProviderMeta {
    PROVIDERS.iter().find(|p| p.provider == provider).expect("provider not in registry")
}

/// Look up provider by name string (case-insensitive, supports aliases).
pub fn provider_by_name(name: &str) -> Option<&'static ProviderMeta> {
    let lower = name.to_lowercase();
    PROVIDERS.iter().find(|p| {
        p.name == lower
            || p.display_name.to_lowercase() == lower
            || match lower.as_str() {
                "grok" => p.provider == ApiProvider::XAI,
                "kimi" => p.provider == ApiProvider::Moonshot,
                "zhipu" => p.provider == ApiProvider::GLM,
                "dashscope" => p.provider == ApiProvider::Qwen,
                "volcengine" | "ark" => p.provider == ApiProvider::Doubao,
                "together-ai" => p.provider == ApiProvider::Together,
                "fireworks-ai" => p.provider == ApiProvider::Fireworks,
                "lm-studio" => p.provider == ApiProvider::LMStudio,
                "llama.cpp" => p.provider == ApiProvider::LlamaCPP,
                _ => false,
            }
    })
}

impl std::fmt::Display for ApiProvider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", provider_meta(*self).display_name)
    }
}
