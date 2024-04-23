const express = require('express');
const fs = require('fs');
const db = require('./db');
const api = express.Router();

const { blacklist, charset } = JSON.parse(fs.readFileSync('./config.json'));

// functions
const rand = () => Math.floor(Math.random() * (charset.lenght - 1));

function isValidURL(url) {
  try {
    new URL(url);
    return true;
  } catch (e) {
    return false;
  }
}

function generateName() {
  let name = '';
  do {
    name = charset[rand()] + charset[rand()];
  } while (db.getUrl(name));
  return name;
}

// api
api.get('/', (req, res) => {
  res.send('<h1>Hello from api!</h1>');
});

api.put('/url', (req, res) => {
  let shortname = req.query.shortname;
  // validate shortname
  if (shortname) {
    if (blacklist.includes(shortname) || db.getUrl(shortname)) {
      return res.send({
        error: 'Error: shortname is used or blacklisted',
        code: 221,
      });
    }
  } else {
    shortname = generateName();
  }
  // validate URL
  if (!isValidURL(req.query.url))
    return res.send({ error: 'Error: URL is invalid', code: 222 });
  // create
  db.createUrl(shortname, req.query.url);
  res.send({
    status: 'Shortcut is created!',
    code: 220,
    shortname: shortname,
  });
});

api.delete('/url', (req, res) => {
  db.deleteUrl(req.query.shortname);
  res.sendStatus(200);
});

module.exports = api;
