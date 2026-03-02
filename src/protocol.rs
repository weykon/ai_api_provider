/// Wire protocol used by a provider.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ApiProtocol {
    /// Anthropic Messages API (x-api-key header)
    Anthropic,
    /// OpenAI Chat Completions format (Bearer auth) — covers most providers
    OpenAiCompat,
    /// Google Gemini generateContent (query param auth)
    Gemini,
}
