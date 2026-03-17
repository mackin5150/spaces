from flask import Flask, jsonify

app = Flask(__name__)


@app.get("/health")
def health():
    return jsonify({"ok": True, "service": "{{name}}"})


if __name__ == "__main__":
    app.run(debug=True, port=5000)
