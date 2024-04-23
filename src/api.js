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

api
  .route('/shortlink')
  .delete((req, res) => {
    db.deleteUrl(req.query.shortname);
    res.sendStatus(200);
  })

  .get((req, res) => {
    const url = db.getUrl(req.query.shortname);
    if (url) res.send({ status: 'URL is found!', code: 200, url: url });
    else res.send({ status: 'URL is not found!', code: 223 });
  })

  .put((req, res) => {
    let shortname = req.query.shortname;
    // validate shortname
    if (shortname) {
      if (blacklist.includes(shortname) || db.getUrl(shortname)) {
        return res.send({
          status: 'Error: shortname is used or blacklisted',
          code: 221,
        });
      }
    } else {
      shortname = generateName();
    }
    // validate URL
    if (!isValidURL(req.query.url))
      return res.send({ status: 'Error: URL is invalid', code: 222 });
    // create
    db.createUrl(shortname, req.query.url);
    res.send({
      status: 'Shortened link is created!',
      code: 220,
      shortname: shortname,
    });
  });

module.exports = api;
