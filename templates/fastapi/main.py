from fastapi import FastAPI

app = FastAPI(title="{{Name}}")


@app.get("/")
def index():
    return {"name": "{{name}}", "status": "ok"}


@app.get("/health")
def health():
    return {"status": "up"}
