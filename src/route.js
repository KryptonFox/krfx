const express = require('express');
const db = require('./db');
const router = express.Router();

router.get('/', (req, res) => {
  res.status(308).redirect('https://krfox.ru/url-shortener')
});

router.get('/ping', (req, res) => {
  res.send('Pong');
});

router.get('/sorry', (req, res) => {
  res.status(404).send('Такой ссылки не существует!')
});

router.get('/krfox', (req, res) => {
  res.status(301).redirect('https://krfox.ru');
});

router.get('/:url', (req, res) => {
  if (db.getUrl(req.params.url)) res.status(301).redirect(db.getUrl(req.params.url));
  else res.status(301).redirect('/sorry');
});

module.exports = router;
