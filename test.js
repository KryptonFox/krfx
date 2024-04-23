fetch('http://localhost:3000/api/url?shortname=tg&url=https://t.me/Krypt0nF', {
  method: 'PUT',
})
  .then((res) => res.text())
  .then((res) => console.log(res));
