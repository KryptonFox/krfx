const db = require('better-sqlite3')('./db.sqlite');

db.prepare(
  'CREATE TABLE IF NOT EXISTS urls (id INTEGER PRIMARY KEY AUTOINCREMENT UNIQUE NOT NULL, shortname TEXT UNIQUE, url TEXT);',
).run();

function getUrl(shortname) {
  return db.prepare('SELECT url FROM urls WHERE shortname = ?').get(shortname)
    ?.url;
}
function createUrl(shortname, url) {
  db.prepare('INSERT INTO urls (shortname, url) VALUES (?, ?)').run(
    shortname,
    url,
  );
}

function deleteUrl(shortname) {
  db.prepare('DELETE FROM urls WHERE shortname = ?').run(shortname);
}

module.exports = {
  getUrl,
  createUrl,
  deleteUrl,
};
