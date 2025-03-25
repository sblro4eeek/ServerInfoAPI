use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about = "A system info server and Telegram bot", long_about = None)]
pub struct Args {
    /// Порт для сервера (по умолчанию: 7878)
    #[arg(long, default_value_t = 7878)]
    pub port: u16,
}