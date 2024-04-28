const express = require('express');
const fs = require('fs');
const cors = require('cors');
const app = express();

const { port } = JSON.parse(fs.readFileSync('./config.json'));

app.use(cors())
app.use('/api', require('./src/api'));
app.use('/', require('./src/route'));

app.listen(port);
