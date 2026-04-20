<script lang="ts">
  import { onMount } from 'svelte';
  
  interface Employee {
    id: string;
    nik?: string;
    name: string;
    position?: string;
    licence?: string[];
  }

  interface Unit {
    id: string;
    cn_unit: string;
    type_unit: string;
    model_unit: string;
    licence?: string;
  }

  interface Assignment {
    id: string;
    employee_id: string;
    employee_name: string;
    unit_id: string;
    cn_unit: string;
    assignment_date: string;
    shift: number;
  }

  let employees = $state<Employee[]>([]);
  let units = $state<Unit[]>([]);
  let assignments = $state<Assignment[]>([]);
  let loading = $state(true);
  let showModal = $state(false);
  
  let form = $state({
    employee_id: '',
    unit_id: '',
    assignment_date: new Date().toISOString().split('T')[0],
    shift: 1
  });

  const API_BASE = `http://${typeof window !== 'undefined' ? window.location.hostname : '127.0.0.1'}:8081`;

  // Derived filtered operators based on position
  let operators = $derived(employees.filter(e => e.position?.toLowerCase() === 'operator'));
  
  // Derived filtered operators based on selected unit's license requirement
  let eligibleOperators = $derived(() => {
    if (!form.unit_id) return operators;
    const selectedUnit = units.find(u => u.id === form.unit_id);
    if (!selectedUnit || !selectedUnit.licence) return operators;
    
    return operators.filter(op => 
      op.licence?.includes(selectedUnit.licence!)
    );
  });

  async function fetchData() {
    loading = true;
    try {
      const [empRes, unitRes, assignRes] = await Promise.all([
        fetch(`${API_BASE}/api/employees`),
        fetch(`${API_BASE}/api/units`),
        fetch(`${API_BASE}/api/assignments`)
      ]);
      
      if (empRes.ok) employees = await empRes.json();
      if (unitRes.ok) units = await unitRes.json();
      if (assignRes.ok) assignments = await assignRes.json();
    } catch (e) {
      console.error(e);
    } finally {
      loading = false;
    }
  }

  async function handleSubmit() {
    try {
      const res = await fetch(`${API_BASE}/api/assignments`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(form)
      });
      if (res.ok) {
        showModal = false;
        fetchData();
      } else {
        const err = await res.text();
        alert(`Error: ${err}`);
      }
    } catch (e) {
      console.error(e);
    }
  }

  async function deleteAssignment(id: string) {
    if (!confirm('Hapus pemasangan ini?')) return;
    try {
      const res = await fetch(`${API_BASE}/api/assignments/${id}`, { method: 'DELETE' });
      if (res.ok) fetchData();
    } catch (e) {
      console.error(e);
    }
  }

  onMount(fetchData);
</script>

<svelte:head>
  <title>Pairing Operator - SJSPAMA</title>
</svelte:head>

<div class="flex justify-between items-center mb-8">
  <div>
    <h2 class="text-3xl font-black text-white tracking-tight">Pairing Operator</h2>
    <p class="text-slate-400 mt-1">Pasangkan Operator dengan Alat sesuai License, Tanggal, dan Shift</p>
  </div>
  <button 
    onclick={() => { showModal = true; form.employee_id = ''; form.unit_id = ''; }}
    class="px-6 py-3 bg-sky-500 text-white rounded-xl font-bold hover:bg-sky-400 transition-all shadow-lg shadow-sky-500/20 flex items-center space-x-2"
  >
    <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" viewBox="0 0 20 20" fill="currentColor">
      <path fill-rule="evenodd" d="M10 3a1 1 0 011 1v5h5a1 1 0 110 2h-5v5a1 1 0 11-2 0v-5H4a1 1 0 110-2h5V4a1 1 0 011-1z" clip-rule="evenodd" />
    </svg>
    <span>Buat Pemasangan</span>
  </button>
</div>

<div class="glass rounded-2xl overflow-hidden border border-[#ffffff10] shadow-2xl">
  <table class="w-full text-left border-collapse">
    <thead>
      <tr class="bg-white/5 border-b border-white/10 uppercase text-[10px] font-black tracking-[0.2em] text-slate-400">
        <th class="px-6 py-5">Tanggal / Shift</th>
        <th class="px-6 py-5">Operator</th>
        <th class="px-6 py-5">Unit (Alat)</th>
        <th class="px-6 py-5 text-right">Actions</th>
      </tr>
    </thead>
    <tbody class="divide-y divide-white/5">
      {#if loading}
        <tr>
          <td colspan="4" class="px-6 py-12 text-center text-slate-500 italic">Memuat data...</td>
        </tr>
      {:else if assignments.length === 0}
        <tr>
          <td colspan="4" class="px-6 py-12 text-center text-slate-500 italic">Belum ada pemasangan operator.</td>
        </tr>
      {:else}
        {#each assignments as assign}
          <tr class="hover:bg-white/5 transition-colors group">
            <td class="px-6 py-4">
              <div class="text-white font-bold">{assign.assignment_date}</div>
              <div class="text-[10px] font-black text-sky-500 uppercase">Shift {assign.shift}</div>
            </td>
            <td class="px-6 py-4">
              <div class="text-white font-medium">{assign.employee_name}</div>
            </td>
            <td class="px-6 py-4">
              <div class="text-white font-black tracking-wider">{assign.cn_unit}</div>
            </td>
            <td class="px-6 py-4 text-right opacity-0 group-hover:opacity-100 transition-opacity">
              <button 
                onclick={() => deleteAssignment(assign.id)}
                class="p-2 text-rose-400 hover:bg-rose-500/10 rounded-lg transition-all"
                title="Hapus Pemasangan"
              >
                <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" />
                </svg>
              </button>
            </td>
          </tr>
        {/each}
      {/if}
    </tbody>
  </table>
</div>

<!-- Modal -->
{#if showModal}
  <div class="fixed inset-0 z-[100] flex items-center justify-center p-4 bg-black/60 backdrop-blur-sm animate-in fade-in duration-200">
    <div class="glass w-full max-w-2xl p-8 rounded-3xl border border-white/20 shadow-2xl scale-in-center overflow-hidden flex flex-col">
      <div class="flex justify-between items-start mb-8 shrink-0">
        <div>
          <div class="flex items-center space-x-3 mb-1">
            <span class="px-2 py-0.5 bg-sky-500/20 text-sky-400 text-[10px] font-black uppercase rounded border border-sky-400/20">Penjadwalan</span>
            <h3 class="text-3xl font-black text-white uppercase tracking-tighter">Pairing Operator</h3>
          </div>
          <p class="text-slate-400 text-sm">Pasangkan operator dengan alat berdasarkan kualifikasi license.</p>
        </div>
        <button 
          onclick={() => { showModal = false; }} 
          class="p-2 bg-white/5 hover:bg-white/10 rounded-xl text-slate-400 hover:text-white transition-all"
          aria-label="Tutup modal"
        >
          <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
          </svg>
        </button>
      </div>
      
      <form onsubmit={(e) => { e.preventDefault(); handleSubmit(); }} class="space-y-8">
        <div class="grid grid-cols-2 gap-6">
          <div>
            <label for="assign_date" class="block text-[10px] font-black text-sky-400 uppercase tracking-widest mb-2 px-1">Tanggal</label>
            <input id="assign_date" type="date" bind:value={form.assignment_date} class="w-full bg-white/5 border border-white/10 rounded-2xl px-5 py-4 text-white focus:outline-none focus:border-sky-500" required>
          </div>
          <div>
            <label for="assign_shift" class="block text-[10px] font-black text-sky-400 uppercase tracking-widest mb-2 px-1">Shift</label>
            <select id="assign_shift" bind:value={form.shift} class="w-full bg-white/10 border border-white/10 rounded-2xl px-5 py-4 text-white focus:outline-none focus:border-sky-500 cursor-pointer">
              <option value={1} class="bg-[#1e293b]">Shift 1 (Siang)</option>
              <option value={2} class="bg-[#1e293b]">Shift 2 (Malam)</option>
            </select>
          </div>
        </div>

        <div class="space-y-6 p-6 bg-white/5 rounded-[2rem] border border-white/10 shadow-inner">
          <div>
            <label for="unit_id" class="block text-[10px] font-black text-slate-400 uppercase tracking-widest mb-3 ml-1">Pilih Unit (Alat)</label>
            <select id="unit_id" bind:value={form.unit_id} class="w-full bg-white/10 border border-white/10 rounded-2xl px-5 py-4 text-white font-bold focus:outline-none focus:border-sky-500 transition-all cursor-pointer" required>
              <option value="" class="bg-[#1e293b]">-- Pilih Alat Berat --</option>
              {#each units as unit}
                <option value={unit.id} class="bg-[#1e293b]">{unit.cn_unit} — {unit.type_unit} ({unit.licence || 'No Licence Req.'})</option>
              {/each}
            </select>
          </div>

          <div>
            <label for="operator_id" class="block text-[10px] font-black text-slate-400 uppercase tracking-widest mb-3 ml-1">
              Pilih Operator 
              {#if form.unit_id}
                <span class="text-sky-400 text-[9px] lowercase italic">(Filtered by Licence)</span>
              {/if}
            </label>
            <select id="operator_id" bind:value={form.employee_id} class="w-full bg-white/10 border border-white/10 rounded-2xl px-5 py-4 text-white font-bold focus:outline-none focus:border-sky-500 transition-all cursor-pointer" required>
              <option value="" class="bg-[#1e293b]">-- Pilih Operator --</option>
              {#each eligibleOperators() as op}
                <option value={op.id} class="bg-[#1e293b]">{op.nik || '-'} — {op.name} ({op.licence?.join(', ') || 'No Licence'})</option>
              {/each}
            </select>
          </div>
        </div>

        <div class="pt-4">
          <button 
            type="submit" 
            class="w-full bg-sky-500 text-white font-black py-5 rounded-2xl hover:bg-sky-400 transition-all shadow-lg shadow-sky-500/20 active:scale-[0.98]"
          >
            SIMPAN PEMASANGAN
          </button>
        </div>
      </form>
    </div>
  </div>
{/if}
