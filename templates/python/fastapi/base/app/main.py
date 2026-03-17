from fastapi import FastAPI

app = FastAPI(title="{{name}}")


@app.get("/health")
def health() -> dict[str, bool]:
    return {"ok": True}
