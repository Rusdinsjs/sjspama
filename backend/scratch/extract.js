const Database = require('d:/Ngoding/sjs-pama/node_modules/better-sqlite3');
const db = new Database('d:/Ngoding/sjs-pama/dev.db');

const rows = db.prepare('SELECT cnUnit, modelUnit, typeUnit, status FROM Unit').all();
console.log(JSON.stringify(rows));
db.close();
