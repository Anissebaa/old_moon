mod analyzer;
mod capture;
mod gui;

#[cfg(feature = "lua")]
mod lua_engine;
#[cfg(feature = "python")]
mod python_engine;
#[cfg(feature = "ruby")]
mod ruby_engine;

use analyzer::PacketAnalyzer;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();

    if let Some(pos) = args.iter().position(|a| a == "--gui") {
        let iface = args.get(pos + 1).map(String::as_str).unwrap_or("eth0");
        let tool = args.get(pos + 2).map(String::as_str).unwrap_or("tshark");
        return match tool {
            "pyshark" => gui::launch_pyshark(iface),
            _         => gui::launch_tshark(iface),
        };
    }

    #[cfg(feature = "lua")]
    {
        let engine = lua_engine::LuaEngine::new("scripts/analyze.lua")?;
        return capture::run(&engine);
    }

    #[cfg(feature = "python")]
    {
        let engine = python_engine::PythonEngine::new("scripts/analyze.py")?;
        return capture::run(&engine);
    }

    #[cfg(feature = "ruby")]
    {
        let engine = ruby_engine::RubyEngine::new("scripts/analyze.rb")?;
        return capture::run(&engine);
    }

    #[allow(unreachable_code)]
    {
        Err("No engine feature enabled. Build with --features lua|python|ruby".into())
    }
}
