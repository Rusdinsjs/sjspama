<script lang="ts">
  import { onMount } from 'svelte';
  
  interface Position {
    id: string;
    name: string;
    created_at?: string;
  }

  let positions = $state<Position[]>([]);
  let loading = $state(true);
  let showModal = $state(false);
  
  let name = $state('');

  const API_BASE = `http://${typeof window !== 'undefined' ? window.location.hostname : '127.0.0.1'}:8081`;

  async function fetchPositions() {
    loading = true;
    try {
      const res = await fetch(`${API_BASE}/api/positions`);
      if (res.ok) positions = await res.json();
    } catch (e) {
      console.error(e);
    } finally {
      loading = false;
    }
  }

  async function handleSubmit() {
    try {
      const res = await fetch(`${API_BASE}/api/positions`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ name })
      });
      if (res.ok) {
        showModal = false;
        name = '';
        fetchPositions();
      }
    } catch (e) {
      console.error(e);
    }
  }

  async function deletePosition(id: string) {
    if (!confirm('Hapus jabatan ini?')) return;
    try {
      const res = await fetch(`${API_BASE}/api/positions/${id}`, { method: 'DELETE' });
      if (res.ok) fetchPositions();
    } catch (e) {
      console.error(e);
    }
  }

  onMount(fetchPositions);
</script>

<svelte:head>
  <title>Master Jabatan - SJSPAMA</title>
</svelte:head>

<div class="flex justify-between items-center mb-8">
  <div>
    <h2 class="text-3xl font-black text-white tracking-tight">Master Jabatan</h2>
    <p class="text-slate-400 mt-1">Kelola daftar jabatan karyawan SJS-PAMA</p>
  </div>
  <button 
    onclick={() => { showModal = true; }}
    class="px-6 py-3 bg-sky-500 text-white rounded-xl font-bold hover:bg-sky-400 transition-all shadow-lg shadow-sky-500/20"
  >
    Tambah Jabatan
  </button>
</div>

<div class="glass rounded-2xl overflow-hidden border border-[#ffffff10] shadow-2xl max-w-2xl">
  <table class="w-full text-left border-collapse">
    <thead>
      <tr class="bg-white/5 border-b border-white/10 uppercase text-[10px] font-black tracking-[0.2em] text-slate-400">
        <th class="px-6 py-5">Nama Jabatan</th>
        <th class="px-6 py-5 text-right">Actions</th>
      </tr>
    </thead>
    <tbody class="divide-y divide-white/5">
      {#if loading}
        <tr>
          <td colspan="2" class="px-6 py-12 text-center text-slate-500 italic">Memuat data...</td>
        </tr>
      {:else if positions.length === 0}
        <tr>
          <td colspan="2" class="px-6 py-12 text-center text-slate-500 italic">Belum ada data jabatan.</td>
        </tr>
      {:else}
        {#each positions as pos}
          <tr class="hover:bg-white/5 transition-colors group">
            <td class="px-6 py-4">
              <span class="text-white font-bold">{pos.name}</span>
            </td>
            <td class="px-6 py-4 text-right opacity-0 group-hover:opacity-100 transition-opacity">
              <button 
                onclick={() => deletePosition(pos.id)}
                class="p-2 text-rose-400 hover:bg-rose-500/10 rounded-lg transition-all"
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

{#if showModal}
  <div class="fixed inset-0 z-[100] flex items-center justify-center p-4 bg-black/60 backdrop-blur-sm animate-in fade-in duration-200">
    <div class="glass w-full max-w-md p-8 rounded-3xl border border-white/20 shadow-2xl scale-in-center overflow-hidden flex flex-col">
      <div class="flex justify-between items-start mb-8 shrink-0">
        <h3 class="text-2xl font-black text-white uppercase tracking-tighter">Tambah Jabatan</h3>
        <button onclick={() => showModal = false} class="text-slate-400 hover:text-white">
          <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
          </svg>
        </button>
      </div>
      
      <form onsubmit={(e) => { e.preventDefault(); handleSubmit(); }} class="space-y-6">
        <div>
          <label for="pos_name" class="block text-[10px] font-black text-sky-400 uppercase tracking-widest mb-2 px-1">Nama Jabatan</label>
          <input id="pos_name" type="text" bind:value={name} placeholder="e.g. Operator, Foreman, Mechanic" class="w-full bg-white/5 border border-white/10 rounded-2xl px-5 py-4 text-white focus:outline-none focus:border-sky-500" required>
        </div>
        <button type="submit" class="w-full bg-sky-500 text-white font-black py-4 rounded-2xl hover:bg-sky-400 transition-all">
          SIMPAN JABATAN
        </button>
      </form>
    </div>
  </div>
{/if}
