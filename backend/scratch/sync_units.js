const Database = require('d:/Ngoding/sjs-pama/node_modules/better-sqlite3');
const db = new Database('d:/Ngoding/sjs-pama/dev.db');

const units = db.prepare('SELECT cnUnit, modelUnit, typeUnit, status FROM Unit').all();
console.log(`Found ${units.length} units to migrate.`);

async function migrate() {
  for (const u of units) {
    const payload = {
      cn_unit: u.cnUnit,
      model_unit: u.modelUnit,
      type_unit: u.typeUnit,
      status: u.status || 'Utama'
    };
    
    try {
      const res = await fetch('http://127.0.0.1:8081/api/units', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(payload)
      });
      if (res.ok) {
        console.log(`Migrated: ${u.cnUnit}`);
      } else {
        console.error(`Failed: ${u.cnUnit} - ${res.status}`);
      }
    } catch (e) {
      console.error(`Error migrating ${u.cnUnit}:`, e.message);
    }
  }
}

migrate();
db.close();
