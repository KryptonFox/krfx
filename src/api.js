const express = require('express');
const fs = require('fs');
const db = require('./db');
const api = express.Router();

const { blacklist, charset } = JSON.parse(fs.readFileSync('./config.json'));

// functions
const rand = () => Math.floor(Math.random() * (charset.length - 1));

function validateUrl(url) {
  try {
    return String(new URL(url));
  } catch (e) {
    try {
      return String(new URL('https://' + url));
    } catch (e) {
      return undefined;
    }
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
  .put((req, res) => {
    let shortname = req.query.shortname;
    // validate shortname
    if (shortname) {
      if (
        shortname.length < 2 ||
        blacklist.includes(shortname) ||
        db.getUrl(shortname)
      ) {
        return res.send({
          status: 'Error: shortname is invalid',
          code: 221,
        });
      }
    } else {
      shortname = generateName();
    }
    // validate URL
    let url = validateUrl(req.query.url);
    if (!url) return res.send({ status: 'Error: URL is invalid', code: 222 });
    // create
    db.createUrl(shortname, url);
    res.send({
      status: 'Shortened link is created!',
      code: 220,
      url: `https://${req.headers.host}/${shortname}`,
    });
  })
  .get((req, res) => {
    const url = db.getUrl(req.query.shortname);
    if (url) res.send({ status: 'URL is found!', code: 200, url: url });
    else res.send({ status: 'URL is not found!', code: 223 });
  })
  .delete((req, res) => {
    db.deleteUrl(req.query.shortname);
    res.sendStatus(200);
  });

module.exports = api;
