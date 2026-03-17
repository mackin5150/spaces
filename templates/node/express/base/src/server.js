require("dotenv").config();
const express = require("express");

const app = express();
const port = Number(process.env.PORT || 3000);

app.use(express.json());

app.get("/health", (_req, res) => {
  res.json({ ok: true, service: "{{name}}" });
});

app.listen(port, () => {
  console.log(`{{name}} listening on http://localhost:${port}`);
});
