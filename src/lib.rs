use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;
use pyo3::types::{PyTuple, PyList};
use clap::Parser;
use serde_json::{Value, to_writer_pretty};
use std::fs::File;
use std::error::Error;

mod args;

use args::Args;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    source: String,
    model_hash: String,
    conf_threshold: f32,
}

pub fn get_args() -> MyResult<Config> {
    let matches = Args::parse();

    Ok(Config{
        source: matches.source,
        model_hash: matches.model,
        conf_threshold: matches.conf,
    })
}

#[allow(deprecated)]
pub fn run(config: Config) -> PyResult<()> {
    Python::with_gil(|py| {
        let sys_path: &PyList = py.import("sys")?.getattr("path")?.downcast()?;
        sys_path.insert(0, "./src")?; // カレントディレクトリ
        //let sys = py.import("sys")?;
        //let path: &PyList = sys.getattr("path")?.downcast()?;
        //println!("sys.path = {:?}", sys);
        // base_tracking モジュールをインポート
        let tracking_module = PyModule::import_bound(py, "tracking")?;

        // Python関数を取得
        let run_tracking = tracking_module.getattr("run_tracking")?;

        // 引数をPythonタプルとして渡す
        let args = PyTuple::new_bound(
            py,
            &[
                config.source.clone().into_py(py),
                config.model_hash.clone().into_py(py),
                config.conf_threshold.into_py(py),
            ],
        );

        // 関数を呼び出し、結果をPyListとして取得
        let result = run_tracking.call1(args)?;
        let result_dict = result.downcast::<PyList>()?;

        // Pythonのjson.dumpsを使ってstrに変換
        let json_module = PyModule::import_bound(py, "json")?;
        let json_str_obj = json_module.getattr("dumps")?.call1((result_dict,))?;
        let json_str: &str = json_str_obj.extract()?;

        // Rust側でserde_json::Valueとしてパース
        let result_json: Value = serde_json::from_str(json_str)
            .map_err(|e| PyValueError::new_err(e.to_string()))?;


        // JSONファイルに保存
        let file = File::create("tracking_result.json").expect("Failed to create file");
        to_writer_pretty(file, &result_json).expect("Failed to write JSON");

        println!("Tracking result saved to tracking_result.json");

        Ok(())
    })
}