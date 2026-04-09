<script lang="ts">
  import { onMount } from 'svelte';
  
  interface Unit {
    id: string;
    cn_unit: string;
    type_unit: string;
    model_unit: string;
    status: string;
    hull_number?: string;
    ct_number?: string;
    ct_expired?: string;
    bast_number?: string;
    skbp_pajak?: string;
  }

  let units = $state<Unit[]>([]);
  let unitLogs = $state<any[]>([]);
  let loading = $state(true);
  let loadingLogs = $state(false);
  let showModal = $state(false);
  let showDeleteConfirm = $state(false);
  let showStatusDropdown = $state(false);
  let modalMode = $state<'create' | 'edit' | 'view'>('create');
  let selectedUnitId = $state<string | null>(null);
  let unitToIdDelete = $state<string | null>(null);
  
  let unitForm = $state({
    cn_unit: '',
    type_unit: '',
    model_unit: '',
    status: 'Utama',
    hull_number: '',
    ct_number: '',
    ct_expired: '',
    bast_number: '',
    skbp_pajak: ''
  });

  const API_BASE = `http://${typeof window !== 'undefined' ? window.location.hostname : '127.0.0.1'}:8081`;

  async function fetchUnits() {
    loading = true;
    try {
      const res = await fetch(`${API_BASE}/api/units`);
      if (res.ok) units = await res.json();
    } catch (e) {
      console.error(e);
    } finally {
      loading = false;
    }
  }

  function openCreate() {
    modalMode = 'create';
    unitForm = { 
      cn_unit: '', type_unit: '', model_unit: '', status: 'Utama',
      hull_number: '', ct_number: '', ct_expired: '', bast_number: '', skbp_pajak: ''
    };
    showModal = true;
  }

  function openEdit(unit: Unit) {
    modalMode = 'edit';
    selectedUnitId = unit.id;
    unitForm = { 
      cn_unit: unit.cn_unit, 
      type_unit: unit.type_unit, 
      model_unit: unit.model_unit, 
      status: unit.status,
      hull_number: unit.hull_number || '',
      ct_number: unit.ct_number || '',
      ct_expired: unit.ct_expired || '',
      bast_number: unit.bast_number || '',
      skbp_pajak: unit.skbp_pajak || ''
    };
    showModal = true;
  }

  async function openView(unit: Unit) {
    modalMode = 'view';
    selectedUnitId = unit.id;
    unitForm = { 
      cn_unit: unit.cn_unit, 
      type_unit: unit.type_unit, 
      model_unit: unit.model_unit, 
      status: unit.status,
      hull_number: unit.hull_number || '-',
      ct_number: unit.ct_number || '-',
      ct_expired: unit.ct_expired || '-',
      bast_number: unit.bast_number || '-',
      skbp_pajak: unit.skbp_pajak || '-'
    };
    showModal = true;
    
    // Fetch productivity history
    loadingLogs = true;
    unitLogs = [];
    try {
      const res = await fetch(`${API_BASE}/api/units/${unit.id}/logs`);
      if (res.ok) unitLogs = await res.json();
    } catch (e) {
      console.error(e);
    } finally {
      loadingLogs = false;
    }
  }

  async function handleSubmit() {
    const url = modalMode === 'create' 
      ? `${API_BASE}/api/units` 
      : `${API_BASE}/api/units/${selectedUnitId}`;
    
    const method = modalMode === 'create' ? 'POST' : 'PUT';

    try {
      const res = await fetch(url, {
        method,
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(unitForm)
      });
      if (res.ok) {
        showModal = false;
        fetchUnits();
      }
    } catch (e) {
      console.error(e);
    }
  }

  async function deleteUnit(id: string) {
    unitToIdDelete = id;
    showDeleteConfirm = true;
  }

  async function confirmDelete() {
    if (!unitToIdDelete) return;
    try {
      const res = await fetch(`${API_BASE}/api/units/${unitToIdDelete}`, { method: 'DELETE' });
      if (res.ok) {
        showDeleteConfirm = false;
        fetchUnits();
      }
    } catch (e) {
      console.error(e);
    }
  }

  onMount(fetchUnits);
</script>

<svelte:head>
  <title>Master Data Alat - SJSPAMA</title>
</svelte:head>

<div class="flex justify-between items-center mb-8">
  <div>
    <h2 class="text-3xl font-black text-white tracking-tight">Master Data Alat</h2>
    <p class="text-slate-400 mt-1">Kelola seluruh armada alat berat SJS-PAMA</p>
  </div>
  <button 
    onclick={openCreate}
    class="px-6 py-3 bg-sky-500 text-white rounded-xl font-bold hover:bg-sky-400 transition-all shadow-lg shadow-sky-500/20 flex items-center space-x-2"
  >
    <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" viewBox="0 0 20 20" fill="currentColor">
      <path fill-rule="evenodd" d="M10 3a1 1 0 011 1v5h5a1 1 0 110 2h-5v5a1 1 0 11-2 0v-5H4a1 1 0 110-2h5V4a1 1 0 011-1z" clip-rule="evenodd" />
    </svg>
    <span>Tambah Alat</span>
  </button>
</div>

<div class="glass rounded-2xl overflow-hidden border border-[#ffffff10] shadow-2xl">
  <table class="w-full text-left border-collapse">
    <thead>
      <tr class="bg-white/5 border-b border-white/10 uppercase text-[10px] font-black tracking-[0.2em] text-slate-400">
        <th class="px-6 py-5">Unit CN</th>
        <th class="px-6 py-5">Type / Model</th>
        <th class="px-6 py-5">Status</th>
        <th class="px-6 py-5 text-right">Actions</th>
      </tr>
    </thead>
    <tbody class="divide-y divide-white/5">
      {#if loading}
        <tr>
          <td colspan="4" class="px-6 py-12 text-center text-slate-500 italic">Loading unit data...</td>
        </tr>
      {:else if units.length === 0}
        <tr>
          <td colspan="4" class="px-6 py-12 text-center text-slate-500 italic">No units found in database.</td>
        </tr>
      {:else}
        {#each units as unit}
          <tr class="hover:bg-white/5 transition-colors group">
            <td class="px-6 py-4">
              <span class="text-white font-bold tracking-wider">{unit.cn_unit}</span>
            </td>
            <td class="px-6 py-4">
              <div class="text-sm font-medium text-white">{unit.type_unit}</div>
              <div class="text-xs text-slate-500">{unit.model_unit}</div>
            </td>
            <td class="px-6 py-4">
              <span class="px-3 py-1 rounded-full text-[10px] font-black uppercase tracking-wider {unit.status === 'Utama' ? 'bg-sky-500/20 text-sky-400 border border-sky-400/20' : 'bg-slate-500/20 text-slate-400 border border-slate-400/20'}">
                {unit.status}
              </span>
            </td>
            <td class="px-6 py-4 text-right opacity-0 group-hover:opacity-100 transition-opacity">
              <div class="flex justify-end space-x-1">
                <button 
                  onclick={() => openView(unit)}
                  class="p-2 text-sky-400 hover:bg-sky-500/10 rounded-lg transition-all"
                  title="View Details"
                >
                  <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" />
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M2.458 12C3.732 7.943 7.523 5 12 5c4.478 0 8.268 2.943 9.542 7-1.274 4.057-5.064 7-9.542 7-4.477 0-8.268-2.943-9.542-7z" />
                  </svg>
                </button>
                <button 
                  onclick={() => openEdit(unit)}
                  class="p-2 text-amber-400 hover:bg-amber-500/10 rounded-lg transition-all"
                  title="Edit Unit"
                >
                  <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z" />
                  </svg>
                </button>
                <button 
                  onclick={() => deleteUnit(unit.id)}
                  class="p-2 text-rose-400 hover:bg-rose-500/10 rounded-lg transition-all"
                  title="Delete Unit"
                >
                  <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" />
                  </svg>
                </button>
              </div>
            </td>
          </tr>
        {/each}
      {/if}
    </tbody>
  </table>
</div>

<!-- Dynamic Modal -->
{#if showModal}
  <div class="fixed inset-0 z-[100] flex items-center justify-center p-4 bg-black/60 backdrop-blur-sm animate-in fade-in duration-200">
    <div class="glass w-full {modalMode === 'view' ? 'max-w-5xl' : 'max-w-3xl'} p-8 rounded-3xl border border-white/20 shadow-2xl scale-in-center overflow-hidden flex flex-col max-h-[90vh]">
      <div class="flex justify-between items-start mb-8 shrink-0">
        <div>
          <div class="flex items-center space-x-3 mb-1">
            <span class="px-2 py-0.5 bg-sky-500/20 text-sky-400 text-[10px] font-black uppercase rounded border border-sky-400/20">Unit Profile</span>
            <h3 class="text-3xl font-black text-white uppercase tracking-tighter">
              {modalMode === 'create' ? 'Tambah Alat Berat' : unitForm.cn_unit}
            </h3>
          </div>
          <p class="text-slate-400 text-sm">
            {#if modalMode === 'create'}Registrasi unit baru ke sistem{:else}{unitForm.type_unit} — {unitForm.model_unit}{/if}
          </p>
        </div>
        <button onclick={() => { showModal = false; showStatusDropdown = false; }} class="p-2 bg-white/5 hover:bg-white/10 rounded-xl text-slate-400 hover:text-white transition-all">
          <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
          </svg>
        </button>
      </div>
      
      {#if modalMode === 'view'}
        <!-- VIEW MODE: DASHBOARD UI -->
        <div class="flex flex-col overflow-hidden space-y-8">
          <!-- Header stats/info -->
          <div class="grid grid-cols-2 md:grid-cols-4 gap-4 shrink-0">
            <div class="p-4 bg-white/5 border border-white/10 rounded-2xl">
              <div class="text-[10px] font-black text-slate-500 uppercase mb-1">No. Lambung</div>
              <div class="text-xl font-black text-white">{unitForm.hull_number}</div>
            </div>
            <div class="p-4 bg-white/5 border border-white/10 rounded-2xl">
              <div class="text-[10px] font-black text-slate-500 uppercase mb-1">Status</div>
              <div class="text-xl font-black text-sky-400">{unitForm.status}</div>
            </div>
            <div class="p-4 bg-white/5 border border-white/10 rounded-2xl">
              <div class="text-[10px] font-black text-slate-500 uppercase mb-1">Expired CT</div>
              <div class="text-xl font-black text-amber-500">{unitForm.ct_expired}</div>
            </div>
            <div class="p-4 bg-white/5 border border-white/10 rounded-2xl">
              <div class="text-[10px] font-black text-slate-500 uppercase mb-1">No. BAST</div>
              <div class="text-xl font-black text-white truncate">{unitForm.bast_number}</div>
            </div>
          </div>

          <!-- Productivity Table -->
          <div class="flex flex-col overflow-hidden bg-black/20 rounded-[2rem] border border-white/5 flex-grow">
            <div class="p-6 border-b border-white/10 flex justify-between items-center">
              <h4 class="text-sm font-black text-white uppercase tracking-widest flex items-center">
                <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4 mr-2 text-sky-500" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 002-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z" />
                </svg>
                Histori Produktivitas
              </h4>
              <span class="text-[10px] text-slate-500 font-bold uppercase tracking-tighter">Last 50 Entries</span>
            </div>

            <div class="overflow-y-auto custom-scrollbar flex-grow">
              <table class="w-full text-left text-xs border-collapse">
                <thead class="sticky top-0 bg-[#0f172a] z-10">
                  <tr class="text-[9px] font-black text-slate-400 uppercase tracking-widest border-b border-white/5">
                    <th class="px-6 py-4">Tgl / Shift</th>
                    <th class="px-6 py-4">Operator</th>
                    <th class="px-6 py-4">Starting HM</th>
                    <th class="px-6 py-4">Total HM</th>
                    <th class="px-6 py-4">WH</th>
                    <th class="px-6 py-4">STB / BD</th>
                  </tr>
                </thead>
                <tbody class="divide-y divide-white/5">
                  {#if loadingLogs}
                    <tr>
                      <td colspan="6" class="px-6 py-20 text-center">
                        <div class="animate-pulse text-sky-400 font-black tracking-widest uppercase text-[10px]">Loading Data Produktivitas...</div>
                      </td>
                    </tr>
                  {:else if unitLogs.length === 0}
                    <tr>
                      <td colspan="6" class="px-6 py-20 text-center text-slate-500 italic">Belum ada data produktivitas tercatat untuk unit ini.</td>
                    </tr>
                  {:else}
                    {#each unitLogs as log}
                      <tr class="hover:bg-white/5 transition-colors">
                        <td class="px-6 py-4">
                          <div class="font-bold text-white">{log.date}</div>
                          <div class="text-[10px] font-black text-sky-500 uppercase">Shift {log.shift}</div>
                        </td>
                        <td class="px-6 py-4 font-medium text-slate-300 uppercase">{log.operator_name}</td>
                        <td class="px-6 py-4 text-slate-400">{log.hm_start}</td>
                        <td class="px-6 py-4">
                          <span class="px-2 py-1 bg-sky-500/10 text-sky-400 rounded-lg font-black">{log.total_hm}</span>
                        </td>
                        <td class="px-6 py-4 text-white font-bold">{log.wh}</td>
                        <td class="px-6 py-4">
                          <span class="text-amber-500">{log.stb}</span>
                          <span class="mx-1 text-slate-700">/</span>
                          <span class="text-rose-500">{log.bd}</span>
                        </td>
                      </tr>
                    {/each}
                  {/if}
                </tbody>
              </table>
            </div>
          </div>
          
          <div class="shrink-0 flex justify-end">
            <button onclick={() => showModal = false} class="px-10 py-4 bg-white/10 text-white font-black rounded-2xl hover:bg-white/20 border border-white/10 transition-all uppercase tracking-widest text-xs">
              Tutup Dashboard
            </button>
          </div>
        </div>
      {:else}
        <!-- CREATE/EDIT MODE: FORM UI -->
        <form onsubmit={(e) => { e.preventDefault(); handleSubmit(); }} class="flex flex-col overflow-hidden">
          <div class="overflow-y-auto pr-2 space-y-6 custom-scrollbar">
            <!-- Primary Info Section -->
            <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
              <div>
                <label for="cn_unit" class="block text-[10px] font-black text-sky-400 uppercase tracking-widest mb-2">Unit CN (Code Number)</label>
                <input 
                  id="cn_unit" 
                  type="text" 
                  bind:value={unitForm.cn_unit} 
                  placeholder="e.g. VISJEX201" 
                  class="w-full bg-white/5 border border-white/10 rounded-2xl px-5 py-3 text-white focus:outline-none focus:border-sky-500" 
                  required
                >
              </div>
              <div>
                <label for="hull_number" class="block text-[10px] font-black text-sky-400 uppercase tracking-widest mb-2">No. Lambung</label>
                <input id="hull_number" type="text" bind:value={unitForm.hull_number} placeholder="e.g. 101" class="w-full bg-white/5 border border-white/10 rounded-2xl px-5 py-3 text-white focus:outline-none focus:border-sky-500">
              </div>
            </div>

            <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
              <div>
                <label for="type_unit" class="block text-[10px] font-black text-slate-400 uppercase tracking-widest mb-2">Unit Type</label>
                <input 
                  id="type_unit" 
                  type="text" 
                  bind:value={unitForm.type_unit} 
                  placeholder="e.g. Excavator" 
                  class="w-full bg-white/5 border border-white/10 rounded-2xl px-5 py-3 text-white focus:outline-none focus:border-sky-500" 
                  required
                >
              </div>
              <div>
                <label for="model_unit" class="block text-[10px] font-black text-slate-400 uppercase tracking-widest mb-2">Unit Model</label>
                <input 
                  id="model_unit" 
                  type="text" 
                  bind:value={unitForm.model_unit} 
                  placeholder="e.g. PC210" 
                  class="w-full bg-white/5 border border-white/10 rounded-2xl px-5 py-3 text-white focus:outline-none focus:border-sky-500" 
                  required
                >
              </div>
            </div>

            <!-- Documentation Section -->
            <div class="p-8 bg-white/5 rounded-[2rem] border border-white/10 space-y-8 shadow-inner shadow-black/20">
              <div class="flex items-center space-x-3 -ml-2 mb-2">
                <div class="w-1 h-4 bg-amber-500 rounded-full"></div>
                <h4 class="text-[10px] font-black text-amber-400 uppercase tracking-[0.2em]">Administrasi & Sertifikasi</h4>
              </div>
              
              <div class="grid grid-cols-1 md:grid-cols-2 gap-8">
                <div>
                  <label for="ct_number" class="block text-[10px] font-black text-slate-400 uppercase tracking-widest mb-3 px-1">No. CT</label>
                  <input id="ct_number" type="text" bind:value={unitForm.ct_number} placeholder="e.g. CT-2024" class="w-full bg-white/5 border border-white/10 rounded-2xl px-5 py-4 text-white focus:outline-none focus:border-amber-500/50 transition-all">
                </div>
                <div>
                  <label for="ct_expired" class="block text-[10px] font-black text-slate-400 uppercase tracking-widest mb-3 px-1">Expired CT</label>
                  <input id="ct_expired" type="date" bind:value={unitForm.ct_expired} class="w-full bg-white/5 border border-white/10 rounded-2xl px-5 py-4 text-white focus:outline-none focus:border-amber-500/50 transition-all">
                </div>
              </div>

              <div class="grid grid-cols-1 md:grid-cols-2 gap-8 border-t border-white/5 pt-8">
                <div>
                  <label for="bast_number" class="block text-[10px] font-black text-slate-400 uppercase tracking-widest mb-3 px-1">No. BAST</label>
                  <input id="bast_number" type="text" bind:value={unitForm.bast_number} placeholder="e.g. BAST/SJS/001" class="w-full bg-white/5 border border-white/10 rounded-2xl px-5 py-4 text-white focus:outline-none focus:border-amber-500/50 transition-all">
                </div>
                <div>
                  <label for="skbp_pajak" class="block text-[10px] font-black text-slate-400 uppercase tracking-widest mb-3 px-1">SKBP (Pajak)</label>
                  <input id="skbp_pajak" type="text" bind:value={unitForm.skbp_pajak} placeholder="e.g. Active" class="w-full bg-white/5 border border-white/10 rounded-2xl px-5 py-4 text-white focus:outline-none focus:border-amber-500/50 transition-all">
                </div>
              </div>
            </div>

            <div class="relative">
              <label for="status" class="block text-[10px] font-black text-slate-400 uppercase tracking-widest mb-2 px-1">Status Armada</label>
              <div class="relative">
                <button 
                  type="button"
                  onclick={() => showStatusDropdown = !showStatusDropdown}
                  class="w-full bg-white/10 border border-white/10 rounded-2xl px-5 py-4 text-white text-left focus:outline-none focus:border-sky-500 flex justify-between items-center transition-all cursor-pointer hover:bg-white/15"
                >
                  <span>{unitForm.status}</span>
                  <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 text-slate-500 transition-transform {showStatusDropdown ? 'rotate-180' : ''}" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7" />
                  </svg>
                </button>

                {#if showStatusDropdown}
                  <div class="absolute z-[110] bottom-full mb-2 w-full glass-dark border border-white/20 rounded-2xl overflow-hidden shadow-2xl animate-in fade-in slide-in-from-bottom-2 duration-200">
                    {#each ['Utama', 'Cadangan', 'Maintenance'] as option}
                      <button 
                        type="button"
                        onclick={() => { unitForm.status = option; showStatusDropdown = false; }}
                        class="w-full px-5 py-4 text-left text-white hover:bg-white/10 transition-colors flex items-center justify-between group"
                      >
                        <span>{option}</span>
                        {#if unitForm.status === option}
                          <div class="w-1.5 h-1.5 bg-sky-500 rounded-full shadow-[0_0_8px_rgba(14,165,233,0.8)]"></div>
                        {/if}
                      </button>
                    {/each}
                  </div>
                {/if}
              </div>
            </div>
          </div>

          <div class="pt-6 shrink-0">
            <button 
              type="submit" 
              class="w-full bg-sky-500 text-white font-black py-4 rounded-2xl hover:bg-sky-400 transition-all shadow-lg shadow-sky-500/20 active:scale-[0.98]"
            >
              {modalMode === 'create' ? 'SIMPAN MASTER DATA' : 'UPDATE DATA UNIT'}
            </button>
          </div>
        </form>
      {/if}
    </div>
  </div>
{/if}

<!-- Confirmation Modal for Delete -->
{#if showDeleteConfirm}
  <div class="fixed inset-0 z-[200] flex items-center justify-center p-4 bg-black/60 backdrop-blur-md animate-in fade-in duration-200">
    <div class="glass max-w-sm w-full p-8 rounded-[2rem] border border-white/20 shadow-2xl scale-in-center flex flex-col items-center text-center">
      <div class="w-20 h-20 bg-rose-500/20 rounded-full flex items-center justify-center mb-6 border border-rose-500/20">
        <svg xmlns="http://www.w3.org/2000/svg" class="h-10 w-10 text-rose-500" fill="none" viewBox="0 0 24 24" stroke="currentColor">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" />
        </svg>
      </div>
      
      <h3 class="text-xl font-black text-white mb-2">Konfirmasi Hapus</h3>
      <p class="text-slate-400 text-sm mb-8 leading-relaxed">
        Apakah Anda yakin ingin menghapus data alat ini? <br>
        <span class="text-rose-400/80 font-medium">Tindakan ini tidak dapat dibatalkan.</span>
      </p>
      
      <div class="grid grid-cols-2 gap-3 w-full">
        <button 
          onclick={() => { showDeleteConfirm = false; unitToIdDelete = null; }}
          class="px-6 py-3 bg-white/5 text-white font-bold rounded-xl hover:bg-white/10 transition-all border border-white/10"
        >
          Batal
        </button>
        <button 
          onclick={confirmDelete}
          class="px-6 py-3 bg-rose-500 text-white font-bold rounded-xl hover:bg-rose-400 transition-all shadow-lg shadow-rose-500/20"
        >
          Ya, Hapus
        </button>
      </div>
    </div>
  </div>
{/if}
