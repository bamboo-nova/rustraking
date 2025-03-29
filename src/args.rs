use clap::Parser;

#[derive(Parser, Clone)]
#[command(name = "track", version = "0.1.0", author = "chiikawa", about = "yolo tracking")]
pub struct Args {
    /// ONNX model path
    #[arg(short = 'm', long = "model", value_name = "MODEL", default_value = "yolov8n", help = "ONNX model path")]
    pub model: String,

    /// input path
    #[arg(value_name = "SOURCE", help = "Input data.", required = true)]
    pub source: String,

    /// using CUDA
    #[arg(long = "cuda", default_value_t = false)]
    pub cuda: bool,

    /// Confidence threshold.
    #[arg(short='c', long="conf-threshold", default_value="0.7")]
    pub conf: f32,
}