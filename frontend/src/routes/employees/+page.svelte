<script lang="ts">
  import { onMount } from 'svelte';

  let employees = $state<any[]>([]);
  let showModal = $state(false);
  let showDeleteModal = $state(false);
  let showDetailModal = $state(false);
  let editingEmployee = $state<any>(null);
  let employeeToDelete = $state<any>(null);
  let viewingEmployee = $state<any>(null);
  let loading = $state(false);

  let form = $state({
    nik: '',
    name: '',
    company: 'SJS',
    position: 'Operator',
    department: 'Production',
    status: 'Active',
    join_date: '',
    contract_type: '',
    ktp_number: '',
    full_address: '',
    village: '',
    district: '',
    city: '',
    province: '',
    origin_status: '',
    gender: 'LAKI-LAKI',
    marital_status: 'S0',
    religion: 'ISLAM',
    birth_place: '',
    birth_date: '',
    education: '',
    email: '',
    phone_number: '',
    simper_number: '',
    simper_expiry: ''
  });

  async function fetchEmployees() {
    try {
      const res = await fetch('http://127.0.0.1:8081/api/employees');
      if (res.ok) employees = await res.json();
    } catch (e) {
      console.error("Failed to fetch employees");
    }
  }

  onMount(fetchEmployees);

  function openCreateModal() {
    editingEmployee = null;
    form = {
      nik: '', name: '', company: 'SJS', position: 'Operator', department: 'Production', status: 'Active', join_date: '',
      contract_type: '', ktp_number: '', full_address: '', village: '', district: '', city: '', province: '',
      origin_status: '', gender: 'LAKI-LAKI', marital_status: 'S0', religion: 'ISLAM', birth_place: '', birth_date: '',
      education: '', email: '', phone_number: '', simper_number: '', simper_expiry: ''
    };
    showModal = true;
  }

  function openEditModal(emp: any) {
    editingEmployee = emp;
    form = {
      nik: emp.nik || '', name: emp.name, company: emp.company || 'SJS', position: emp.position || '', 
      department: emp.department || '', status: emp.status || 'Active', join_date: emp.join_date || '',
      contract_type: emp.contract_type || '', ktp_number: emp.ktp_number || '', full_address: emp.full_address || '', 
      village: emp.village || '', district: emp.district || '', city: emp.city || '', province: emp.province || '',
      origin_status: emp.origin_status || '', gender: emp.gender || 'LAKI-LAKI', marital_status: emp.marital_status || 'S0', 
      religion: emp.religion || 'ISLAM', birth_place: emp.birth_place || '', birth_date: emp.birth_date || '',
      education: emp.education || '', email: emp.email || '', phone_number: emp.phone_number || '', 
      simper_number: emp.simper_number || '', simper_expiry: emp.simper_expiry || ''
    };
    showModal = true;
  }

  function openDetail(emp: any) {
    viewingEmployee = emp;
    showDetailModal = true;
  }

  function confirmDelete(emp: any) {
    employeeToDelete = emp;
    showDeleteModal = true;
  }

  async function saveEmployee() {
    loading = true;
    const url = editingEmployee 
      ? `http://127.0.0.1:8081/api/employees/${editingEmployee.id}` 
      : 'http://127.0.0.1:8081/api/employees';
    
    try {
      const res = await fetch(url, {
        method: editingEmployee ? 'PUT' : 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(form)
      });
      if (res.ok) {
        showModal = false;
        fetchEmployees();
      }
    } catch (e) {
      console.error("Save error");
    }
    loading = false;
  }

  async function deleteEmployee() {
    if (!employeeToDelete) return;
    try {
      const res = await fetch(`http://127.0.0.1:8081/api/employees/${employeeToDelete.id}`, {
        method: 'DELETE'
      });
      if (res.ok) {
        showDeleteModal = false;
        fetchEmployees();
      }
    } catch (e) {
      console.error("Delete error");
    }
  }
</script>

<svelte:head>
  <title>Master Karyawan - SJS PAMA</title>
</svelte:head>

<div class="space-y-8 animate-in fade-in duration-700">
  <div class="flex justify-between items-center">
    <div>
      <h2 class="text-3xl font-black text-white tracking-tight">Master Karyawan</h2>
      <p class="text-slate-400 mt-1">Database personalia (Operator, Driver, Mechanic)</p>
    </div>
    <button 
      onclick={openCreateModal}
      class="bg-sky-500 hover:bg-sky-400 text-white px-8 py-3 rounded-2xl font-black uppercase text-xs tracking-widest transition-all shadow-lg shadow-sky-500/20 active:scale-95"
    >
      Tambah Karyawan
    </button>
  </div>

  <div class="glass rounded-[2rem] overflow-hidden border border-white/10 shadow-2xl">
    <table class="w-full text-left">
      <thead>
        <tr class="bg-white/5 border-b border-white/10">
          <th class="px-8 py-6 text-[10px] font-black text-slate-400 uppercase tracking-widest">NIK</th>
          <th class="px-8 py-6 text-[10px] font-black text-slate-400 uppercase tracking-widest">Nama Lengkap</th>
          <th class="px-8 py-6 text-[10px] font-black text-slate-400 uppercase tracking-widest">Jabatan</th>
          <th class="px-8 py-6 text-[10px] font-black text-slate-400 uppercase tracking-widest">Dept</th>
          <th class="px-8 py-6 text-[10px] font-black text-slate-400 uppercase tracking-widest">Company</th>
          <th class="px-8 py-6 text-[10px] font-black text-slate-400 uppercase tracking-widest">Status</th>
          <th class="px-8 py-6 text-[10px] font-black text-slate-400 uppercase tracking-widest text-right">Aksi</th>
        </tr>
      </thead>
      <tbody class="divide-y divide-white/5">
        {#each employees as emp}
          <tr class="hover:bg-white/5 transition-colors group">
            <td class="px-8 py-5 font-mono text-xs text-sky-400 font-bold">{emp.nik || 'N/A'}</td>
            <td class="px-8 py-5 text-white font-bold">{emp.name}</td>
            <td class="px-8 py-5">
              <span class="px-3 py-1 bg-slate-500/10 border border-slate-500/20 text-slate-400 rounded-full text-[10px] font-black uppercase tracking-wider">
                {emp.position || 'OPERATOR'}
              </span>
            </td>
            <td class="px-8 py-5 text-slate-400 text-sm">{emp.department || '-'}</td>
            <td class="px-8 py-5">
              <span class="font-black text-[10px] {emp.company === 'SJS' ? 'text-sky-400' : 'text-amber-500'}">
                {emp.company}
              </span>
            </td>
            <td class="px-8 py-5">
              <div class="flex items-center space-x-2">
                <div class="w-1.5 h-1.5 rounded-full {emp.status === 'Active' ? 'bg-emerald-500' : 'bg-rose-500'}"></div>
                <span class="text-xs {emp.status === 'Active' ? 'text-emerald-500' : 'text-rose-500'} font-bold">{emp.status}</span>
              </div>
            </td>
            <td class="px-8 py-5 text-right">
              <div class="flex justify-end space-x-2 opacity-0 group-hover:opacity-100 transition-opacity">
                <button onclick={() => openDetail(emp)} class="p-2 hover:bg-emerald-500/10 text-emerald-400 rounded-lg transition-colors" title="View Detail">
                  <svg xmlns="http://www.w3.org/2000/svg" class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" /><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M2.458 12C3.732 7.943 7.523 5 12 5c4.478 0 8.268 2.943 9.542 7-1.274 4.057-5.064 7-9.542 7-4.477 0-8.268-2.943-9.542-7z" /></svg>
                </button>
                <button onclick={() => openEditModal(emp)} class="p-2 hover:bg-sky-500/10 text-sky-400 rounded-lg transition-colors" title="Edit">
                  <svg xmlns="http://www.w3.org/2000/svg" class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z" /></svg>
                </button>
                <button onclick={() => confirmDelete(emp)} class="p-2 hover:bg-rose-500/10 text-rose-400 rounded-lg transition-colors" title="Delete">
                  <svg xmlns="http://www.w3.org/2000/svg" class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" /></svg>
                </button>
              </div>
            </td>
          </tr>
        {/each}
      </tbody>
    </table>
  </div>
</div>

<!-- Modal Create/Edit -->
{#if showModal}
  <div class="fixed inset-0 z-50 flex items-center justify-center p-4 animate-in fade-in duration-300">
    <div class="absolute inset-0 bg-slate-950/80 backdrop-blur-sm" onclick={() => showModal = false}></div>
    <div class="glass-dark w-full max-w-4xl max-h-[90vh] rounded-[3rem] border border-white/10 shadow-2xl relative z-10 flex flex-col">
      <div class="p-10 pb-0">
        <h3 class="text-2xl font-black text-white mb-8 tracking-tight">
          {editingEmployee ? 'Edit Data Karyawan' : 'Tambah Karyawan Baru'}
        </h3>
      </div>
      
      <div class="flex-1 overflow-y-auto p-10 pt-0 custom-scroll">
        <div class="grid grid-cols-1 md:grid-cols-2 gap-x-8 gap-y-6">
          <!-- Section: Basic Info -->
          <div class="col-span-1 md:col-span-2 py-4 border-b border-white/5 mb-2">
            <span class="text-[10px] font-black text-sky-400 uppercase tracking-[0.3em]">Informasi Dasar</span>
          </div>
          <div>
            <label class="block text-[10px] font-black text-slate-500 uppercase tracking-widest mb-2 ml-1">NIK</label>
            <input type="text" bind:value={form.nik} class="w-full bg-white/5 border border-white/10 rounded-2xl px-5 py-4 text-white focus:outline-none focus:border-sky-500">
          </div>
          <div>
            <label class="block text-[10px] font-black text-slate-500 uppercase tracking-widest mb-2 ml-1">Nama Lengkap</label>
            <input type="text" bind:value={form.name} class="w-full bg-white/5 border border-white/10 rounded-2xl px-5 py-4 text-white focus:outline-none focus:border-sky-500" required>
          </div>
          <div>
            <label class="block text-[10px] font-black text-slate-500 uppercase tracking-widest mb-2 ml-1">KTP Number</label>
            <input type="text" bind:value={form.ktp_number} class="w-full bg-white/5 border border-white/10 rounded-2xl px-5 py-4 text-white focus:outline-none focus:border-sky-500">
          </div>
          <div>
            <label class="block text-[10px] font-black text-slate-500 uppercase tracking-widest mb-2 ml-1">Gender</label>
            <select bind:value={form.gender} class="w-full bg-white/5 border border-white/10 rounded-2xl px-5 py-4 text-white focus:outline-none focus:border-sky-500">
              <option value="LAKI-LAKI" class="bg-[#1e293b]">Laki-laki</option>
              <option value="PEREMPUAN" class="bg-[#1e293b]">Perempuan</option>
            </select>
          </div>

          <!-- Section: Employment Info -->
          <div class="col-span-1 md:col-span-2 py-4 border-b border-white/5 mt-4 mb-2">
            <span class="text-[10px] font-black text-amber-400 uppercase tracking-[0.3em]">Data Pekerjaan</span>
          </div>
          <div>
            <label class="block text-[10px] font-black text-slate-500 uppercase tracking-widest mb-2 ml-1">Jabatan</label>
            <input type="text" bind:value={form.position} class="w-full bg-white/5 border border-white/10 rounded-2xl px-5 py-4 text-white focus:outline-none focus:border-sky-500">
          </div>
          <div>
            <label class="block text-[10px] font-black text-slate-500 uppercase tracking-widest mb-2 ml-1">Departemen</label>
            <input type="text" bind:value={form.department} class="w-full bg-white/5 border border-white/10 rounded-2xl px-5 py-4 text-white focus:outline-none focus:border-sky-500">
          </div>
          <div>
            <label class="block text-[10px] font-black text-slate-500 uppercase tracking-widest mb-2 ml-1">Perusahaan</label>
            <select bind:value={form.company} class="w-full bg-white/5 border border-white/10 rounded-2xl px-5 py-4 text-white">
              <option value="SJS" class="bg-[#1e293b]">SJS (Satria Jaya Sultra)</option>
              <option value="PAMA" class="bg-[#1e293b]">PAMA (Pama Persada)</option>
            </select>
          </div>
          <div>
            <label class="block text-[10px] font-black text-slate-500 uppercase tracking-widest mb-2 ml-1">Contract Type</label>
            <input type="text" bind:value={form.contract_type} class="w-full bg-white/5 border border-white/10 rounded-2xl px-5 py-4 text-white">
          </div>

          <!-- Section: Personal Detail -->
          <div class="col-span-1 md:col-span-2 py-4 border-b border-white/5 mt-4 mb-2">
            <span class="text-[10px] font-black text-emerald-400 uppercase tracking-[0.3em]">Detail Pribadi & Kontak</span>
          </div>
          <div>
            <label class="block text-[10px] font-black text-slate-500 uppercase tracking-widest mb-2 ml-1">Phone Number</label>
            <input type="text" bind:value={form.phone_number} class="w-full bg-white/5 border border-white/10 rounded-2xl px-5 py-4 text-white">
          </div>
          <div>
            <label class="block text-[10px] font-black text-slate-500 uppercase tracking-widest mb-2 ml-1">Email</label>
            <input type="email" bind:value={form.email} class="w-full bg-white/5 border border-white/10 rounded-2xl px-5 py-4 text-white">
          </div>
          <div class="col-span-1 md:col-span-2">
            <label class="block text-[10px] font-black text-slate-500 uppercase tracking-widest mb-2 ml-1">Full Address</label>
            <textarea bind:value={form.full_address} class="w-full bg-white/5 border border-white/10 rounded-2xl px-5 py-4 text-white resize-none h-24"></textarea>
          </div>
        </div>
      </div>

      <div class="p-10 pt-4 flex justify-end space-x-4 border-t border-white/5">
        <button onclick={() => showModal = false} class="px-8 py-4 text-slate-400 font-black uppercase text-[10px] tracking-widest hover:text-white transition-colors">Batal</button>
        <button onclick={saveEmployee} disabled={loading} class="bg-sky-500 hover:bg-sky-400 text-white px-10 py-4 rounded-2xl font-black uppercase text-[10px] tracking-widest transition-all shadow-xl shadow-sky-500/20 active:scale-95 disabled:opacity-50">
          {loading ? 'Menyimpan...' : 'Simpan Data'}
        </button>
      </div>
    </div>
  </div>
{/if}

<!-- Detail Modal -->
{#if showDetailModal && viewingEmployee}
  <div class="fixed inset-0 z-50 flex items-center justify-center p-4 animate-in fade-in duration-300">
    <div class="absolute inset-0 bg-slate-950/90 backdrop-blur-md" onclick={() => showDetailModal = false}></div>
    <div class="glass-dark w-full max-w-5xl max-h-[90vh] rounded-[3rem] border border-white/10 shadow-2xl relative z-10 flex flex-col overflow-hidden">
      <!-- Header -->
      <div class="p-10 bg-gradient-to-br from-sky-500/10 to-transparent flex justify-between items-start">
        <div class="flex items-center space-x-8">
          <div class="w-24 h-24 rounded-3xl bg-sky-500/20 border-2 border-sky-500/30 flex items-center justify-center text-4xl font-black text-sky-400 shadow-xl shadow-sky-500/10">
            {viewingEmployee?.name?.[0]?.toUpperCase() || '?'}
          </div>
          <div class="space-y-2">
            <h3 class="text-3xl font-black text-white tracking-tighter">{viewingEmployee?.name || 'No Name'}</h3>
            <div class="flex items-center space-x-3">
              <span class="px-3 py-1 bg-sky-500/10 border border-sky-500/20 text-sky-400 rounded-lg text-[10px] font-black uppercase tracking-widest">
                {viewingEmployee?.nik || 'NO NIK'}
              </span>
              <span class="px-3 py-1 bg-white/5 border border-white/10 text-slate-400 rounded-lg text-[10px] font-black uppercase tracking-widest">
                {viewingEmployee?.position || 'STAF'}
              </span>
            </div>
          </div>
        </div>
        <button onclick={() => showDetailModal = false} class="p-4 bg-white/5 hover:bg-white/10 text-white rounded-2xl transition-all">
          <svg xmlns="http://www.w3.org/2000/svg" class="w-6 h-6" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" /></svg>
        </button>
      </div>

      <!-- Content -->
      <div class="flex-1 overflow-y-auto p-10 pt-0 custom-scroll">
        <div class="grid grid-cols-1 md:grid-cols-3 gap-10">
          <!-- Col 1: Bio -->
          <div class="space-y-8">
            <div>
              <p class="text-[10px] font-black text-slate-500 uppercase tracking-widest mb-4">Biodata Diri</p>
              <div class="space-y-4">
                <div class="p-4 bg-white/5 rounded-2xl flex justify-between items-center">
                  <span class="text-[10px] font-medium text-slate-400 uppercase">KTP</span>
                  <span class="text-xs font-bold text-white">{viewingEmployee?.ktp_number || '-'}</span>
                </div>
                <div class="p-4 bg-white/5 rounded-2xl flex justify-between items-center">
                  <span class="text-[10px] font-medium text-slate-400 uppercase">Gender</span>
                  <span class="text-xs font-bold text-white">{viewingEmployee?.gender || '-'}</span>
                </div>
                <div class="p-4 bg-white/5 rounded-2xl flex justify-between items-center">
                  <span class="text-[10px] font-medium text-slate-400 uppercase">Status</span>
                  <span class="text-xs font-bold text-white">{viewingEmployee?.marital_status || '-'}</span>
                </div>
                <div class="p-4 bg-white/5 rounded-2xl flex justify-between items-center">
                  <span class="text-[10px] font-medium text-slate-400 uppercase">Agama</span>
                  <span class="text-xs font-bold text-white">{viewingEmployee?.religion || '-'}</span>
                </div>
              </div>
            </div>
          </div>

          <!-- Col 2: Job & Skills -->
          <div class="space-y-8">
            <div>
              <p class="text-[10px] font-black text-amber-500 uppercase tracking-widest mb-4">Pekerjaan & Sertifikasi</p>
              <div class="space-y-4">
                <div class="p-6 bg-amber-500/5 border border-amber-500/10 rounded-2xl space-y-4">
                  <div>
                    <span class="text-[9px] font-bold text-amber-500/60 uppercase block mb-1">Join Date</span>
                    <span class="text-sm font-black text-white">{viewingEmployee?.join_date || '-'}</span>
                  </div>
                  <div>
                    <span class="text-[9px] font-bold text-amber-500/60 uppercase block mb-1">Kontrak</span>
                    <span class="text-sm font-black text-white">{viewingEmployee?.contract_type || '-'}</span>
                  </div>
                </div>
                <div class="p-6 bg-emerald-500/5 border border-emerald-500/10 rounded-2xl space-y-4">
                  <div>
                    <span class="text-[9px] font-bold text-emerald-500/60 uppercase block mb-1">No. SIMPER</span>
                    <span class="text-sm font-black text-white">{viewingEmployee?.simper_number || '-'}</span>
                  </div>
                  <div>
                    <span class="text-[9px] font-bold text-emerald-500/60 uppercase block mb-1">Masa Berlaku</span>
                    <span class="text-sm font-black text-white text-emerald-400">{viewingEmployee?.simper_expiry || '-'}</span>
                  </div>
                </div>
              </div>
            </div>
          </div>

          <!-- Col 3: Contact -->
          <div class="space-y-8">
            <div>
              <p class="text-[10px] font-black text-sky-500 uppercase tracking-widest mb-4">Kontak & Alamat</p>
              <div class="space-y-4">
                <div class="p-4 bg-white/5 rounded-2xl">
                  <span class="text-[9px] font-medium text-slate-400 uppercase block mb-1">Telepon / WhatsApp</span>
                  <span class="text-sm font-bold text-sky-400">{viewingEmployee?.phone_number || '-'}</span>
                </div>
                <div class="p-4 bg-white/5 rounded-2xl">
                  <span class="text-[9px] font-medium text-slate-400 uppercase block mb-1">Email</span>
                  <span class="text-sm font-bold text-white">{viewingEmployee?.email || '-'}</span>
                </div>
                <div class="p-4 bg-white/5 rounded-2xl h-32 overflow-y-auto">
                  <span class="text-[9px] font-medium text-slate-400 uppercase block mb-1">Alamat Domisili</span>
                  <p class="text-xs leading-relaxed text-slate-300">{viewingEmployee?.full_address || '-'}</p>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
{/if}

<!-- Modal Delete -->
{#if showDeleteModal}
  <div class="fixed inset-0 z-50 flex items-center justify-center p-4 animate-in zoom-in duration-200">
    <div class="absolute inset-0 bg-slate-950/80 backdrop-blur-sm" onclick={() => showDeleteModal = false}></div>
    <div class="glass-dark w-full max-w-sm rounded-[2.5rem] p-8 border border-white/10 shadow-2xl relative z-10 text-center">
      <div class="w-16 h-16 bg-rose-500/20 text-rose-500 rounded-full flex items-center justify-center mx-auto mb-6">
        <svg xmlns="http://www.w3.org/2000/svg" class="w-8 h-8" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z" /></svg>
      </div>
      <h3 class="text-xl font-black text-white mb-2 tracking-tight">Hapus Karyawan?</h3>
      <p class="text-slate-400 text-sm mb-8 leading-relaxed">
        Anda akan menghapus data <span class="text-white font-bold">{employeeToDelete.name}</span>. Tindakan ini tidak dapat dibatalkan.
      </p>
      <div class="grid grid-cols-2 gap-4">
        <button onclick={() => showDeleteModal = false} class="px-6 py-4 bg-white/5 hover:bg-white/10 text-white rounded-2xl font-black uppercase text-[10px] tracking-widest transition-all">Batal</button>
        <button onclick={deleteEmployee} class="px-6 py-4 bg-rose-500 hover:bg-rose-400 text-white rounded-2xl font-black uppercase text-[10px] tracking-widest transition-all shadow-lg shadow-rose-500/20">Hapus</button>
      </div>
    </div>
  </div>
{/if}

<style>
  .glass {
    background: rgba(15, 23, 42, 0.4);
    backdrop-filter: blur(20px);
    -webkit-backdrop-filter: blur(20px);
  }
  .glass-dark {
    background: rgba(15, 23, 42, 0.95);
    backdrop-filter: blur(20px);
    -webkit-backdrop-filter: blur(20px);
  }
  .custom-scroll::-webkit-scrollbar {
    width: 6px;
  }
  .custom-scroll::-webkit-scrollbar-thumb {
    background: rgba(255,255,255,0.1);
    border-radius: 10px;
  }
</style>

