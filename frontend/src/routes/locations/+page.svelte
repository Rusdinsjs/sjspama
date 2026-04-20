<script lang="ts">
  import { onMount } from 'svelte';
  
  interface WorkLocation {
    id: string;
    name: string;
    description?: string;
    created_at?: string;
  }

  let locations = $state<WorkLocation[]>([]);
  let loading = $state(true);
  let showModal = $state(false);
  let showDeleteConfirm = $state(false);
  let modalMode = $state<'create' | 'edit'>('create');
  let selectedLocationId = $state<string | null>(null);
  let locationToIdDelete = $state<string | null>(null);
  
  let locationForm = $state({
    name: '',
    description: ''
  });

  const API_BASE = `http://${typeof window !== 'undefined' ? window.location.hostname : '127.0.0.1'}:8081`;

  async function fetchLocations() {
    loading = true;
    try {
      const res = await fetch(`${API_BASE}/api/work-locations`);
      if (res.ok) locations = await res.json();
    } catch (e) {
      console.error(e);
    } finally {
      loading = false;
    }
  }

  function openCreate() {
    modalMode = 'create';
    locationForm = { name: '', description: '' };
    showModal = true;
  }

  function openEdit(loc: WorkLocation) {
    modalMode = 'edit';
    selectedLocationId = loc.id;
    locationForm = { 
      name: loc.name, 
      description: loc.description || ''
    };
    showModal = true;
  }

  async function handleSubmit() {
    const url = modalMode === 'create' 
      ? `${API_BASE}/api/work-locations` 
      : `${API_BASE}/api/work-locations/${selectedLocationId}`;
    
    const method = modalMode === 'create' ? 'POST' : 'PUT';

    try {
      const res = await fetch(url, {
        method,
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(locationForm)
      });
      if (res.ok) {
        showModal = false;
        fetchLocations();
      }
    } catch (e) {
      console.error(e);
    }
  }

  async function deleteLocation(id: string) {
    locationToIdDelete = id;
    showDeleteConfirm = true;
  }

  async function confirmDelete() {
    if (!locationToIdDelete) return;
    try {
      const res = await fetch(`${API_BASE}/api/work-locations/${locationToIdDelete}`, { method: 'DELETE' });
      if (res.ok) {
        showDeleteConfirm = false;
        fetchLocations();
      }
    } catch (e) {
      console.error(e);
    }
  }

  onMount(fetchLocations);
</script>

<svelte:head>
  <title>Master Lokasi Kerja - SJSPAMA</title>
</svelte:head>

<div class="flex justify-between items-center mb-8">
  <div>
    <h2 class="text-3xl font-black text-white tracking-tight">Master Lokasi Kerja</h2>
    <p class="text-slate-400 mt-1">Kelola daftar lokasi kerja untuk input Daily Log</p>
  </div>
  <button 
    onclick={openCreate}
    class="px-6 py-3 bg-sky-500 text-white rounded-xl font-bold hover:bg-sky-400 transition-all shadow-lg shadow-sky-500/20 flex items-center space-x-2"
  >
    <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" viewBox="0 0 20 20" fill="currentColor">
      <path fill-rule="evenodd" d="M10 3a1 1 0 011 1v5h5a1 1 0 110 2h-5v5a1 1 0 11-2 0v-5H4a1 1 0 110-2h5V4a1 1 0 011-1z" clip-rule="evenodd" />
    </svg>
    <span>Tambah Lokasi</span>
  </button>
</div>

<div class="glass rounded-2xl overflow-hidden border border-[#ffffff10] shadow-2xl">
  <table class="w-full text-left border-collapse">
    <thead>
      <tr class="bg-white/5 border-b border-white/10 uppercase text-[10px] font-black tracking-[0.2em] text-slate-400">
        <th class="px-6 py-5">Nama Lokasi</th>
        <th class="px-6 py-5">Deskripsi</th>
        <th class="px-6 py-5 text-right">Actions</th>
      </tr>
    </thead>
    <tbody class="divide-y divide-white/5">
      {#if loading}
        <tr>
          <td colspan="3" class="px-6 py-12 text-center text-slate-500 italic">Loading location data...</td>
        </tr>
      {:else if locations.length === 0}
        <tr>
          <td colspan="3" class="px-6 py-12 text-center text-slate-500 italic">Belum ada lokasi kerja terdaftar.</td>
        </tr>
      {:else}
        {#each locations as loc}
          <tr class="hover:bg-white/5 transition-colors group">
            <td class="px-6 py-4">
              <span class="text-white font-bold tracking-wider">{loc.name}</span>
            </td>
            <td class="px-6 py-4">
              <span class="text-slate-400 text-sm">{loc.description || '-'}</span>
            </td>
            <td class="px-6 py-4 text-right opacity-0 group-hover:opacity-100 transition-opacity">
              <div class="flex justify-end space-x-1">
                <button 
                  onclick={() => openEdit(loc)}
                  class="p-2 text-amber-400 hover:bg-amber-500/10 rounded-lg transition-all"
                  title="Edit Lokasi"
                >
                  <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z" />
                  </svg>
                </button>
                <button 
                  onclick={() => deleteLocation(loc.id)}
                  class="p-2 text-rose-400 hover:bg-rose-500/10 rounded-lg transition-all"
                  title="Hapus Lokasi"
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

<!-- Modal -->
{#if showModal}
  <div class="fixed inset-0 z-[100] flex items-center justify-center p-4 bg-black/60 backdrop-blur-sm animate-in fade-in duration-200">
    <div class="glass w-full max-w-lg p-8 rounded-3xl border border-white/20 shadow-2xl scale-in-center overflow-hidden flex flex-col">
      <div class="flex justify-between items-start mb-8 shrink-0">
        <div>
          <div class="flex items-center space-x-3 mb-1">
            <span class="px-2 py-0.5 bg-sky-500/20 text-sky-400 text-[10px] font-black uppercase rounded border border-sky-400/20">Lokasi Kerja</span>
            <h3 class="text-3xl font-black text-white uppercase tracking-tighter">
              {modalMode === 'create' ? 'Tambah Lokasi' : 'Edit Lokasi'}
            </h3>
          </div>
          <p class="text-slate-400 text-sm">
            {modalMode === 'create' ? 'Daftarkan lokasi kerja baru untuk operasional' : 'Perbarui informasi lokasi kerja'}
          </p>
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
      
      <form onsubmit={(e) => { e.preventDefault(); handleSubmit(); }} class="space-y-6">
        <div>
          <label for="name" class="block text-[10px] font-black text-sky-400 uppercase tracking-widest mb-2">Nama Lokasi</label>
          <input 
            id="name" 
            type="text" 
            bind:value={locationForm.name} 
            placeholder="e.g. PIT 1, Disposal, ROM" 
            class="w-full bg-white/5 border border-white/10 rounded-2xl px-5 py-3 text-white focus:outline-none focus:border-sky-500" 
            required
          >
        </div>
        <div>
          <label for="description" class="block text-[10px] font-black text-slate-400 uppercase tracking-widest mb-2">Deskripsi (Opsional)</label>
          <textarea 
            id="description" 
            bind:value={locationForm.description} 
            placeholder="Keterangan tambahan mengenai lokasi..." 
            class="w-full bg-white/5 border border-white/10 rounded-2xl px-5 py-3 text-white focus:outline-none focus:border-sky-500 min-h-[100px]"
          ></textarea>
        </div>

        <div class="pt-6 shrink-0">
          <button 
            type="submit" 
            class="w-full bg-sky-500 text-white font-black py-4 rounded-2xl hover:bg-sky-400 transition-all shadow-lg shadow-sky-500/20 active:scale-[0.98]"
          >
            {modalMode === 'create' ? 'SIMPAN LOKASI' : 'UPDATE LOKASI'}
          </button>
        </div>
      </form>
    </div>
  </div>
{/if}

<!-- Delete Confirmation -->
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
        Apakah Anda yakin ingin menghapus lokasi ini? <br>
        <span class="text-rose-400/80 font-medium">Ini dapat mempengaruhi data historis jika lokasi ini sudah digunakan.</span>
      </p>
      
      <div class="grid grid-cols-2 gap-3 w-full">
        <button 
          onclick={() => { showDeleteConfirm = false; locationToIdDelete = null; }}
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
