use rig::providers::ollama;
use rig::streaming::{stream_to_stdout, StreamingPrompt};
use anyhow::Result;

/// 创建一个新的Ollama客户端并将响应流式输出到标准输出
/// 
/// # 参数
/// * `model_name` - 要使用的模型名称
/// * `temperature` - 用于生成的温度参数
/// * `prompt` - 发送给模型的提示词
/// 
/// # 返回值
/// * `Result<()>` - 成功返回Ok，否则返回Err
pub async fn new_ollama(model_name: &str, temperature: f64, prompt: &str) -> Result<()> {
    // 使用单一上下文提示创建流式代理
    let agent = ollama::Client::new()
        .agent(model_name)
        .preamble("准确的回答我.")
        .temperature(temperature)
        .build();

    // 流式输出响应并在接收到数据块时打印
    let mut stream = agent
        .stream_prompt(prompt)
        .await?;

    stream_to_stdout(agent, &mut stream).await?;

    Ok(())
}