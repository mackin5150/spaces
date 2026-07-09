use anyhow::{Context, Result};
use std::fs;
use std::path::Path;

const MAX_NEW_TOKENS: usize = 256;

/// Determine if a string looks like a HuggingFace repo ID (contains '/')
fn is_hf_repo(model_ref: &str) -> bool {
    model_ref.contains('/') && !model_ref.starts_with('.') && !model_ref.starts_with('/')
}

pub fn run(
    path: String,
    name: Option<String>,
    model: String,
    install: bool,
    docker: bool,
    cuda: bool,
) -> Result<()> {
    let root = Path::new(&path);
    let project_name = match &name {
        Some(n) => n.clone(),
        None => root
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("iface-project")
            .to_string(),
    };

    let model_source = if is_hf_repo(&model) {
        format!("\"{}\"", model)
    } else {
        format!("'./models/{}'", model)
    };

    // .spaces/workspace.yaml
    fs::create_dir_all(root.join(".spaces"))?;
    let mut features = String::from("- iface\n");
    if docker {
        features.push_str("- docker\n");
    }
    if cuda {
        features.push_str("- cuda\n");
    }
    fs::write(
        root.join(".spaces/workspace.yaml"),
        format!(
            "name: {}\nlanguage: python\nstack: iface\nversion: 2\npackage_manager: uv\nmodel: {}\ncreated_with: spaces 0.1.0\nfeatures:\n{}",
            project_name, model, features.trim()
        ),
    )?;

    // pyproject.toml
    fs::write(
        root.join("pyproject.toml"),
        r#"[project]
name = "{{NAME}}"
version = "0.1.0"
requires-python = ">=3.10"
dependencies = [
    "fastapi>=0.115",
    "uvicorn[standard]>=0.34",
    "pydantic>=2.0",
    "transformers>=4.48",
    "torch>=2.5",
    "huggingface-hub>=0.27",
]

[project.scripts]
iface = "server:main"
"#
        .replace("{{NAME}}", &project_name),
    )?;

    // server.py — FastAPI inference server with streaming SSE chat UI
    fs::write(
        root.join("server.py"),
        r#"import os
import uvicorn
from contextlib import asynccontextmanager
from fastapi import FastAPI, Request
from fastapi.responses import HTMLResponse, StreamingResponse, JSONResponse
from pydantic import BaseModel
from transformers import AutoModelForCausalLM, AutoTokenizer

MODEL = "{{MODEL}}"
PORT = int(os.getenv("IFACE_PORT", "7860"))
MAX_NEW_TOKENS = int(os.getenv("MAX_NEW_TOKENS", "256"))


@asynccontextmanager
async def lifespan(app: FastAPI):
    global tokenizer, model
    print(f"Loading model {MODEL} ...")
    tokenizer = AutoTokenizer.from_pretrained(MODEL)
    model = AutoModelForCausalLM.from_pretrained(
        MODEL,
        torch_dtype="auto",
        device_map="auto",
    )
    print(f"Ready on http://localhost:{PORT}")


app = FastAPI(title="{{NAME}} Inference Interface", lifespan=lifespan)

tokenizer: AutoTokenizer | None = None
model: AutoModelForCausalLM | None = None


class ChatRequest(BaseModel):
    prompt: str
    max_tokens: int = {{MAX_NEW_TOKENS}}


@app.get("/", response_class=HTMLResponse)
async def index():
    return open("index.html").read()


@app.post("/api/chat")
async def chat(req: Request):
    data = await req.json()
    prompt = data.get("prompt", "")
    max_tokens = data.get("max_tokens", {{MAX_NEW_TOKENS}})

    if not tokenizer or not model:
        return JSONResponse({"error": "model not loaded"}, status_code=503)

    inputs = tokenizer(prompt, return_tensors="pt").to(model.device)

    def generate():
        for token_id in model.generate(
            **inputs,
            max_new_tokens=max_tokens,
            do_sample=True,
            temperature=0.7,
            top_p=0.95,
            pad_token_id=tokenizer.eos_token_id,
        ):
            text = tokenizer.decode(token_id[0], skip_special_tokens=True)
            yield f"data: {text!r}\n\n"

    return StreamingResponse(generate(), media_type="text/event-stream")


@app.post("/api/completions")
async def completions(req: Request):
    data = await req.json()
    prompt = data.get("prompt", "")
    max_tokens = data.get("max_tokens", {{MAX_NEW_TOKENS}})

    if not tokenizer or not model:
        return JSONResponse({"error": "model not loaded"}, status_code=503)

    inputs = tokenizer(prompt, return_tensors="pt").to(model.device)
    output = model.generate(
        **inputs,
        max_new_tokens=max_tokens,
        do_sample=True,
        temperature=0.7,
        top_p=0.95,
        pad_token_id=tokenizer.eos_token_id,
    )
    text = tokenizer.decode(output[0], skip_special_tokens=True)

    return JSONResponse({"text": text})


@app.get("/api/health")
async def health():
    return {"status": "ok", "model": MODEL}


def main():
    uvicorn.run("server:app", host="0.0.0.0", port=PORT, reload=True)


if __name__ == "__main__":
    main()
"#
        .replace("{{NAME}}", &project_name)
        .replace("{{MODEL}}", &model_source)
        .replace("{{MAX_NEW_TOKENS}}", &MAX_NEW_TOKENS.to_string()),
    )?;

    // index.html — minimal chat UI
    fs::write(
        root.join("index.html"),
        r#"<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="UTF-8">
<meta name="viewport" content="width=device-width, initial-scale=1.0">
<title>{{NAME}} - Inference Interface</title>
<style>
  * { box-sizing: border-box; margin: 0; padding: 0; }
  body { font-family: system-ui, sans-serif; background: #0f0f0f; color: #e4e4e4; display: flex; flex-direction: column; height: 100vh; }
  header { padding: 16px 24px; border-bottom: 1px solid #333; display: flex; align-items: center; gap: 12px; }
  header h1 { font-size: 18px; font-weight: 600; }
  #status { font-size: 12px; color: #4ade80; padding: 2px 8px; border-radius: 99px; background: rgba(74,222,128,.1); }
  #chat { flex: 1; overflow-y: auto; padding: 24px; display: flex; flex-direction: column; gap: 16px; }
  .msg { max-width: 70%; padding: 12px 16px; border-radius: 16px; line-height: 1.5; white-space: pre-wrap; word-break: break-word; font-size: 14px; }
  .user { align-self: flex-end; background: #2563eb; color: #fff; border-bottom-right-radius: 4px; }
  .assistant { align-self: flex-start; background: #1e1e1e; border: 1px solid #333; border-bottom-left-radius: 4px; }
  #input-area { padding: 16px 24px; display: flex; gap: 8px; border-top: 1px solid #333; background: #1a1a1a; }
  textarea { flex: 1; resize: none; height: 48px; padding: 10px 14px; border-radius: 12px; border: 1px solid #444; background: #0f0f0f; color: #e4e4e4; font-family: inherit; font-size: 14px; outline: none; }
  textarea:focus { border-color: #2563eb; }
  button { padding: 0 20px; border-radius: 12px; border: none; background: #2563eb; color: #fff; font-weight: 600; cursor: pointer; height: 48px; }
  button:hover { background: #1d4ed8; }
  button.loading { opacity: .6; pointer-events: none; }
</style>
</head>
<body>
<header><h1>{{NAME}}</h1><span id="status">● Connected</span></header>
<div id="chat"></div>
<div id="input-area">
  <textarea id="prompt" placeholder="Type a message..." rows="1"></textarea>
  <button id="send">Send</button>
</div>

<script>
const chat = document.getElementById('chat');
const promptEl = document.getElementById('prompt');
const sendBtn = document.getElementById('send');

function addMsg(role, text) {
  const d = document.createElement('div');
  d.className = 'msg ' + role;
  d.textContent = text;
  chat.appendChild(d);
  chat.scrollTop = chat.scrollHeight;
  return d;
}

async function send() {
  const text = promptEl.value.trim();
  if (!text) return;
  addMsg('user', text);
  promptEl.value = '';
  sendBtn.classList.add('loading');

  const el = addMsg('assistant', '');
  try {
    const resp = await fetch('/api/chat', {
      method: 'POST',
      headers: {'Content-Type': 'application/json'},
      body: JSON.stringify({prompt: text}),
    });
    if (resp.body) {
      const reader = resp.body.getReader();
      const dec = new TextDecoder();
      let buf = '';
      while (true) {
        const {done, value} = await reader.read();
        buf += dec.decode(value, {stream: !done});
        const lines = buf.split('\n\n');
        buf = lines.pop() || '';
        for (const line of lines) {
          if (line.startsWith('data: ')) {
            el.textContent = JSON.parse(line.slice(6));
            chat.scrollTop = chat.scrollHeight;
          }
        }
        if (done) break;
      }
    } else {
      const json = await resp.json();
      el.textContent = json.text || json.error || '';
    }
  } catch (e) {
    el.textContent = 'Error: ' + e.message;
  }
  sendBtn.classList.remove('loading');
}

sendBtn.onclick = send;
promptEl.onkeydown = e => { if (e.key === 'Enter' && !e.shiftKey) { e.preventDefault(); send(); }};
</script>
</body>
</html>"#
        .replace("{{NAME}}", &project_name),
    )?;

    // run.sh
    fs::write(
        root.join("run.sh"),
        format!(
            r#"#!/usr/bin/env bash
set -e

echo "Starting {{NAME}} inference interface..."
echo "Model: {model}"

if command -v uv &>/dev/null; then
    uv run python server.py
else
    pip install fastapi uvicorn transformers torch huggingface-hub pydantic
    python server.py
fi
"#
        ),
    )?;
    fs::set_permissions(root.join("run.sh"), std::os::unix::fs::PermissionsExt::from_mode(0o755))?;

    // Dockerfile (optional)
    if docker {
        let base_image = if cuda { "nvidia/cuda:12.4-runtime-ubuntu22.04" } else { "python:3.12-slim" };
        fs::write(
            root.join("Dockerfile"),
            format!(
                r#"FROM {}

RUN pip install --no-cache-dir fastapi uvicorn transformers torch huggingface-hub pydantic

WORKDIR /app
COPY . .

EXPOSE 7860
CMD ["python", "server.py"]
"#
            , base_image),
        )?;
    }

    // .env.example
    fs::write(
        root.join(".env.example"),
        format!(
            r#"IFACE_PORT=7860
MAX_NEW_TOKENS=256
MODEL={model}
HUGGING_FACE_HUB_TOKEN=
"#
        ),
    )?;

    println!("\n  iface workspace scaffolded:\n");
    println!("   {}           | {}", project_name, "project name");
    println!("   {}           | {}", model, "model (HF repo or local path)");
    println!("   uv            | package manager");
    println!();

    if is_hf_repo(&model) {
        println!(
            "  TIP: Run `huggingface-cli login` first to authenticate with HuggingFace Hub.\n"
        );
    } else {
        println!(
            "  TIP: Model will be loaded from ./models/{}/ on startup.\n",
            model
        );
    }

    if install {
        println!("  Installing dependencies...");
        let status = std::process::Command::new("uv")
            .arg("sync")
            .current_dir(root)
            .status()
            .context("uv sync failed — make sure 'uv' is installed")?;

        if status.success() {
            println!("  Dependencies installed.\n");
        } else {
            println!("  uv sync exited with code {:?}\n", status.code());
        }
    }

    println!("  To start the interface:");
    println!("    bash run.sh");
    println!("    or: uv run python server.py\n");

    Ok(())
}
