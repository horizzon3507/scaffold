from flask import Flask, jsonify

app = Flask(__name__)


@app.get("/")
def index():
    return jsonify(name="{{name}}", status="ok")


@app.get("/health")
def health():
    return jsonify(status="up")


if __name__ == "__main__":
    app.run(debug=True)
