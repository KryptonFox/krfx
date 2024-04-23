const express = require('express');
const app = express();
const fs = require('fs');

const { port } = JSON.parse(fs.readFileSync('./config.json'));

app.use('/api', require('./src/api'));
app.use('/', require('./src/route'));

app.listen(port);
