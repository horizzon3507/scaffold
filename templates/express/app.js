const express = require('express');

const app = express();
app.use(express.json());

app.get('/', (_req, res) => {
  res.json({ name: '{{name}}', status: 'ok' });
});

app.get('/health', (_req, res) => {
  res.json({ status: 'up' });
});

const port = process.env.PORT || 3000;
app.listen(port, () => {
  console.log(`{{name}} on http://localhost:${port}`);
});
