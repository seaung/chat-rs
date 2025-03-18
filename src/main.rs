use anyhow::Result;
use std::io::{self, Write};
mod app;
use app::chat;

#[tokio::main]
async fn main() -> Result<()> {
    println!("欢迎使用AI聊天程序！输入问题开始对话，输入'bye'退出程序。");
    
    loop {
        print!("问题> ");
        io::stdout().flush()?;
        
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        
        let input = input.trim();
        
        if input.to_lowercase() == "bye" {
            println!("感谢使用，再见！");
            break;
        }
        
        if !input.is_empty() {
            println!("AI正在思考...");
            chat::new_ollama("qwen2:7b", 0.5, input).await?;
            println!();
        }
    }
    
    Ok(())
}
