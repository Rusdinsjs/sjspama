<script lang="ts">
  import { onMount } from 'svelte';

  let units: any[] = [];
  let showStb = $state(false);
  let showBd = $state(false);

  let form = $state({
    unit_id: '',
    date: new Date().toISOString().split('T')[0],
    shift: 1,
    operator_name: '',
    work_location: '',
    hm_start: 0,
    hm_stop: 0,
    wh: 0,
    stb: 0,
    stb_start: 0,
    stb_stop: 0,
    stb_remarks: '',
    bd: 0,
    bd_start: 0,
    bd_stop: 0,
    bd_remarks: '',
    bd_job_desc: '',
    bd_status: 'Continue',
    remarks: ''
  });

  let total_hm = $derived(form.hm_stop - form.hm_start);
  
  // Effect to auto-calculate STB/BD totals
  $effect(() => {
    if (showStb) form.stb = Math.max(0, form.stb_stop - form.stb_start);
    if (showBd) form.bd = Math.max(0, form.bd_stop - form.bd_start);
  });

  let status_msg = $state<{type: 'success' | 'error', text: string} | null>(null);
  let loading = $state(false);

  onMount(async () => {
    try {
      const res = await fetch('http://127.0.0.1:8081/api/units');
      if (res.ok) units = await res.json();
    } catch (e) {
      console.error("Could not load units");
    }
  });

  async function submitForm(e: SubmitEvent) {
    e.preventDefault();
    loading = true;
    status_msg = null;

    if (form.wh + form.stb + form.bd > 12) {
      status_msg = { type: 'error', text: 'Total WH + STB + BD cannot exceed 12 hours (MOHH)' };
      loading = false;
      return;
    }

    try {
      // Clean up values if section is hidden
      const submissionData = { ...form };
      if (!showStb) {
        submissionData.stb = 0; submissionData.stb_start = 0; submissionData.stb_stop = 0; submissionData.stb_remarks = '';
      }
      if (!showBd) {
        submissionData.bd = 0; submissionData.bd_start = 0; submissionData.bd_stop = 0; 
        submissionData.bd_remarks = ''; submissionData.bd_job_desc = ''; submissionData.bd_status = '';
      }

      const res = await fetch('http://127.0.0.1:8081/api/daily-logs', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(submissionData)
      });
      const data = await res.json();
      if (res.ok) {
        status_msg = { type: 'success', text: 'Log entry successfully saved!' };
        form.hm_start = form.hm_stop;
        form.hm_stop = 0; form.wh = 0; form.stb = 0; form.bd = 0;
        showStb = false; showBd = false;
      } else {
        status_msg = { type: 'error', text: data.message || 'Failed to save log' };
      }
    } catch (e) {
      status_msg = { type: 'error', text: 'Failed to connect to backend server.' };
    }
    loading = false;
  }
</script>

<svelte:head>
  <title>Daily Input - SJS PAMA</title>
</svelte:head>

<div class="mb-8">
  <h2 class="text-3xl font-black text-white tracking-tight">Daily Log Input</h2>
  <p class="text-slate-400 mt-1">Pencatatan aktivitas operasional harian armada</p>
</div>

<div class="glass p-8 rounded-[2.5rem] max-w-5xl shadow-2xl relative overflow-hidden border border-white/10">
  {#if status_msg}
    <div class="mb-8 p-5 rounded-2xl flex items-center {status_msg.type === 'success' ? 'bg-sky-500/20 text-sky-400 border border-sky-500/30' : 'bg-rose-500/20 text-rose-400 border border-rose-500/30'} animate-in fade-in slide-in-from-top-4">
      <p class="font-black uppercase text-xs tracking-widest">{status_msg.text}</p>
    </div>
  {/if}

  <form onsubmit={submitForm} class="space-y-10">
    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-8">
      <!-- Row 1: Time context -->
      <div class="lg:col-span-2 grid grid-cols-2 gap-6 p-6 bg-white/5 rounded-3xl border border-white/5">
        <div>
          <label class="block text-[10px] font-black text-slate-500 uppercase tracking-[0.2em] mb-3 ml-1">Tanggal Kerja</label>
          <input type="date" bind:value={form.date} class="w-full bg-white/5 border border-white/10 rounded-2xl px-5 py-4 text-white focus:outline-none focus:border-sky-500 transition-all" required>
        </div>
        <div>
          <label class="block text-[10px] font-black text-slate-500 uppercase tracking-[0.2em] mb-3 ml-1">Shift</label>
          <select bind:value={form.shift} class="w-full bg-white/10 border border-white/10 rounded-2xl px-5 py-4 text-white focus:outline-none focus:border-sky-500 transition-all cursor-pointer">
            <option value={1} class="bg-[#1e293b]">Shift 1 (Siang)</option>
            <option value={2} class="bg-[#1e293b]">Shift 2 (Malam)</option>
          </select>
        </div>
      </div>

      <div class="p-6 bg-sky-500/5 rounded-3xl border border-sky-500/10 flex flex-col justify-center">
        <label class="block text-[10px] font-black text-sky-400 uppercase tracking-[0.2em] mb-3 ml-1">Unit CN</label>
        <select bind:value={form.unit_id} class="w-full bg-white/10 border border-white/10 rounded-2xl px-5 py-4 text-white font-bold focus:outline-none focus:border-sky-500 transition-all cursor-pointer" required>
          <option value="" class="bg-[#1e293b]">-- Pilih Unit --</option>
          {#each units as unit}
            <option value={unit.id} class="bg-[#1e293b]">{unit.cn_unit} — {unit.model_unit}</option>
          {/each}
        </select>
      </div>

      <!-- Row 2: Personnel & Location -->
      <div class="md:col-span-2 grid grid-cols-1 md:grid-cols-2 gap-8">
        <div>
          <label class="block text-[10px] font-black text-slate-400 uppercase tracking-[0.2em] mb-3 ml-1">Nama Operator</label>
          <input type="text" bind:value={form.operator_name} placeholder="Masukkan nama operator..." class="w-full bg-white/5 border border-white/10 rounded-2xl px-6 py-4 text-white focus:outline-none focus:border-sky-500 transition-all" required>
        </div>
        <div>
          <label class="block text-[10px] font-black text-slate-400 uppercase tracking-[0.2em] mb-3 ml-1">Lokasi Kerja</label>
          <input type="text" bind:value={form.work_location} placeholder="Contoh: Disposal A, Pit North..." class="w-full bg-white/5 border border-white/10 rounded-2xl px-6 py-4 text-white focus:outline-none focus:border-sky-500 transition-all" required>
        </div>
      </div>

      <!-- HM Counter -->
      <div class="p-6 bg-white/5 rounded-3xl border border-white/5 flex flex-col space-y-4">
        <div class="grid grid-cols-2 gap-4">
          <div>
            <label class="block text-[9px] font-black text-sky-500 uppercase mb-2">HM Start</label>
            <input type="number" step="0.1" bind:value={form.hm_start} class="w-full bg-white/5 border border-white/10 rounded-xl px-4 py-3 text-white text-sm focus:outline-none focus:border-sky-500" required>
          </div>
          <div>
            <label class="block text-[9px] font-black text-sky-500 uppercase mb-2">HM Stop</label>
            <input type="number" step="0.1" bind:value={form.hm_stop} class="w-full bg-white/5 border border-white/10 rounded-xl px-4 py-3 text-white text-sm focus:outline-none focus:border-sky-500" required>
          </div>
        </div>
        <div class="pt-2">
          <label class="block text-[9px] font-black text-slate-500 uppercase mb-2 ml-1">Total Penggunaan HM</label>
          <div class="w-full bg-sky-500/10 border border-sky-500/30 rounded-2xl px-5 py-4 text-sky-400 font-black text-lg">
            {total_hm > 0 ? total_hm.toFixed(2) : '0.00'}
          </div>
        </div>
      </div>
    </div>

    <!-- Operational Hours Toggles -->
    <div class="space-y-8">
      <div class="flex flex-wrap gap-4">
        <div class="w-full md:w-48">
          <label class="block text-[10px] font-black text-blue-400 uppercase tracking-widest mb-3 ml-1">Working Hours (WH)</label>
          <input type="number" step="0.1" bind:value={form.wh} class="w-full bg-white/10 border border-white/10 rounded-2xl px-6 py-4 text-white font-black text-xl focus:outline-none focus:border-blue-500 shadow-lg shadow-blue-500/5">
        </div>
        
        <div class="flex-grow grid grid-cols-2 gap-4">
          <button 
            type="button" 
            onclick={() => showStb = !showStb}
            class="px-6 py-4 rounded-2xl border font-black uppercase text-xs tracking-widest transition-all flex items-center justify-center space-x-3 
            {showStb ? 'bg-amber-500 text-white border-amber-400 shadow-lg shadow-amber-500/20' : 'bg-white/5 text-amber-500 border-amber-500/20 hover:bg-amber-500/10'}"
          >
            <div class="w-2 h-2 rounded-full {showStb ? 'bg-white animate-pulse' : 'bg-amber-500'}"></div>
            <span>Standby (STB)</span>
          </button>

          <button 
            type="button" 
            onclick={() => showBd = !showBd}
            class="px-6 py-4 rounded-2xl border font-black uppercase text-xs tracking-widest transition-all flex items-center justify-center space-x-3 
            {showBd ? 'bg-rose-500 text-white border-rose-400 shadow-lg shadow-rose-500/20' : 'bg-white/5 text-rose-500 border-rose-500/20 hover:bg-rose-500/10'}"
          >
            <div class="w-2 h-2 rounded-full {showBd ? 'bg-white animate-pulse' : 'bg-rose-500'}"></div>
            <span>Breakdown (BD)</span>
          </button>
        </div>
      </div>

      <!-- Conditional Sections -->
      {#if showStb}
        <div class="p-8 bg-amber-500/5 border border-amber-500/20 rounded-[2rem] space-y-6 animate-in slide-in-from-top-4 duration-300">
          <h4 class="text-amber-500 font-black uppercase text-[10px] tracking-[0.3em] flex items-center">
            <span class="w-8 h-[1px] bg-amber-500/30 mr-3"></span>
            Detail Durasi Standby
          </h4>
          <div class="grid grid-cols-1 md:grid-cols-4 gap-6 items-end">
            <div>
              <label class="block text-[9px] font-bold text-slate-500 uppercase mb-2">STB Start</label>
              <input type="number" step="0.1" bind:value={form.stb_start} class="w-full bg-white/5 border border-white/10 rounded-xl px-4 py-3 text-white focus:border-amber-500">
            </div>
            <div>
              <label class="block text-[9px] font-bold text-slate-500 uppercase mb-2">STB Stop</label>
              <input type="number" step="0.1" bind:value={form.stb_stop} class="w-full bg-white/5 border border-white/10 rounded-xl px-4 py-3 text-white focus:border-amber-500">
            </div>
            <div>
              <label class="block text-[9px] font-bold text-amber-500 uppercase mb-2">Total STB</label>
              <input type="number" step="0.1" bind:value={form.stb} class="w-full bg-amber-500/10 border border-amber-500/30 rounded-xl px-4 py-3 text-amber-400 font-black">
            </div>
            <div class="md:col-span-1">
              <label class="block text-[9px] font-bold text-slate-500 uppercase mb-2">Keterangan STB</label>
              <input type="text" bind:value={form.stb_remarks} placeholder="e.g. Hujan, No Operator..." class="w-full bg-white/5 border border-white/10 rounded-xl px-4 py-3 text-white focus:border-amber-500">
            </div>
          </div>
        </div>
      {/if}

      {#if showBd}
        <div class="p-8 bg-rose-500/5 border border-rose-500/20 rounded-[2rem] space-y-8 animate-in slide-in-from-top-4 duration-300">
          <h4 class="text-rose-500 font-black uppercase text-[10px] tracking-[0.3em] flex items-center">
            <span class="w-8 h-[1px] bg-rose-500/30 mr-3"></span>
            Laporan Kerusakan & Perbaikan (Breakdown)
          </h4>
          
          <div class="grid grid-cols-1 md:grid-cols-4 gap-6 items-end">
            <div>
              <label class="block text-[9px] font-bold text-slate-500 uppercase mb-2">BD Start</label>
              <input type="number" step="0.1" bind:value={form.bd_start} class="w-full bg-white/5 border border-white/10 rounded-xl px-4 py-3 text-white focus:border-rose-500">
            </div>
            <div>
              <label class="block text-[9px] font-bold text-slate-500 uppercase mb-2">BD Stop</label>
              <input type="number" step="0.1" bind:value={form.bd_stop} class="w-full bg-white/5 border border-white/10 rounded-xl px-4 py-3 text-white focus:border-rose-500">
            </div>
            <div>
              <label class="block text-[9px] font-bold text-rose-500 uppercase mb-2">Total BD</label>
              <input type="number" step="0.1" bind:value={form.bd} class="w-full bg-rose-500/10 border border-rose-500/30 rounded-xl px-4 py-3 text-rose-400 font-black">
            </div>
            <div>
              <label class="block text-[9px] font-bold text-slate-500 uppercase mb-2">Status Perbaikan</label>
              <select bind:value={form.bd_status} class="w-full bg-white/10 border border-white/10 rounded-xl px-4 py-3 text-white focus:border-rose-500 cursor-pointer">
                <option value="Done" class="bg-[#1e293b]">DONE (Selesai)</option>
                <option value="Continue" class="bg-[#1e293b]">CONTINUE (Lanjut)</option>
              </select>
            </div>
          </div>

          <div class="grid grid-cols-1 md:grid-cols-2 gap-8">
            <div>
              <label class="block text-[10px] font-black text-slate-400 uppercase tracking-widest mb-3 ml-1">Keterangan Kerusakan</label>
              <textarea bind:value={form.bd_remarks} rows="2" class="w-full bg-white/5 border border-white/10 rounded-2xl px-5 py-4 text-white focus:border-rose-500 resize-none" placeholder="Jelaskan komponen yang rusak..."></textarea>
            </div>
            <div>
              <label class="block text-[10px] font-black text-slate-400 uppercase tracking-widest mb-3 ml-1">Deskripsi Pekerjaan (Desc Job)</label>
              <textarea bind:value={form.bd_job_desc} rows="2" class="w-full bg-white/5 border border-white/10 rounded-2xl px-5 py-4 text-white focus:border-rose-500 resize-none" placeholder="Tindakan perbaikan yang dilakukan..."></textarea>
            </div>
          </div>
        </div>
      {/if}
    </div>

    <!-- General Remarks -->
    <div class="pt-4 border-t border-white/5">
      <label class="block text-[10px] font-black text-slate-500 uppercase tracking-[0.2em] mb-3 ml-1">Catatan Tambahan (General Remarks)</label>
      <textarea bind:value={form.remarks} rows="2" class="w-full bg-white/5 border border-white/10 rounded-3xl px-6 py-5 text-white focus:outline-none focus:border-sky-500 transition-all resize-none" placeholder="Tambahkan konteks teknis lainnya jika diperlukan..."></textarea>
    </div>

    <div class="flex justify-end pt-4">
      <button 
        type="submit" 
        disabled={loading} 
        class="w-full md:w-auto px-12 py-5 bg-sky-500 hover:bg-sky-400 text-white rounded-2xl font-black uppercase tracking-[0.2em] text-sm transition-all shadow-xl shadow-sky-500/20 active:scale-95 disabled:opacity-50"
      >
        {loading ? 'Sedang Menyimpan...' : 'Submit Daily Log'}
      </button>
    </div>
  </form>
</div>

<style>
  .glass {
    background: var(--bg-card);
    backdrop-filter: var(--glass-blur);
    -webkit-backdrop-filter: var(--glass-blur);
  }
</style>
