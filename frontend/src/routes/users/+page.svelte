<script lang="ts">
  import { onMount } from 'svelte';

  let users = $state<any[]>([]);
  let showCreateModal = $state(false);
  let loading = $state(false);

  let newUser = $state({
    name: '',
    email: '',
    password: '',
    role: 'OPERATOR'
  });

  async function fetchUsers() {
    try {
      const res = await fetch('http://127.0.0.1:8081/api/users');
      if (res.ok) users = await res.json();
    } catch (e) {
      console.error(e);
    }
  }

  async function createUser() {
    loading = true;
    try {
      const res = await fetch('http://127.0.0.1:8081/api/users', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(newUser)
      });
      if (res.ok) {
        showCreateModal = false;
        newUser = { name: '', email: '', password: '', role: 'OPERATOR' };
        fetchUsers();
      }
    } catch (e) {
      console.error(e);
    } finally {
      loading = false;
    }
  }

  async function deleteUser(id: string) {
    if (!confirm('Are you sure you want to delete this user?')) return;
    try {
      const res = await fetch(`http://127.0.0.1:8081/api/users/${id}`, { method: 'DELETE' });
      if (res.ok) fetchUsers();
    } catch (e) {
      console.error(e);
    }
  }

  onMount(fetchUsers);
</script>

<svelte:head>
  <title>User Management - SJS PAMA</title>
</svelte:head>

<div class="mb-8 flex justify-between items-center">
  <div>
    <h2 class="text-3xl font-bold text-white tracking-wide">User Management</h2>
    <p class="text-slate-400 mt-1">Manage system access roles and credentials</p>
  </div>
  <button 
    onclick={() => showCreateModal = true}
    class="px-6 py-3 bg-sky-500 text-white rounded-xl font-bold hover:bg-sky-400 transition-all shadow-lg shadow-sky-500/20"
  >
    Add New User
  </button>
</div>

<div class="glass rounded-2xl overflow-hidden border border-white/10 shadow-2xl bg-white/5">
  <div class="p-6 border-b border-white/5 bg-white/5">
    <h3 class="text-xl font-bold text-white">System Users</h3>
  </div>
  
  <div class="overflow-x-auto">
    <table class="w-full text-left text-sm">
      <thead class="text-slate-400 bg-white/5 border-b border-white/10">
        <tr>
          <th class="px-6 py-4 font-semibold uppercase tracking-wider">User</th>
          <th class="px-6 py-4 font-semibold uppercase tracking-wider">Role</th>
          <th class="px-6 py-4 font-semibold uppercase tracking-wider text-right">Actions</th>
        </tr>
      </thead>
      <tbody class="divide-y divide-white/5">
        {#each users as user}
          <tr class="hover:bg-white/5 transition-colors">
            <td class="px-6 py-4">
              <div class="flex items-center space-x-3">
                <div class="w-10 h-10 rounded-full bg-slate-800 flex items-center justify-center font-bold text-sky-500 border border-sky-500/20">
                  {user.name?.[0].toUpperCase() || 'U'}
                </div>
                <div>
                  <p class="font-bold text-white leading-tight">{user.name}</p>
                  <p class="text-xs text-slate-500 leading-tight">{user.email}</p>
                </div>
              </div>
            </td>
            <td class="px-6 py-4">
              <span class="px-3 py-1 rounded-full text-xs font-bold {user.role === 'ADMIN' ? 'bg-indigo-500/20 text-indigo-400 border border-indigo-400/20' : 'bg-sky-500/20 text-sky-400 border border-sky-400/20'}">
                {user.role}
              </span>
            </td>
            <td class="px-6 py-4 text-right">
              <button 
                onclick={() => deleteUser(user.id)}
                aria-label={`Delete user ${user.name}`}
                class="text-rose-500 hover:text-rose-400 font-bold transition-colors p-2 hover:bg-rose-500/10 rounded-lg"
              >
                Delete
              </button>
            </td>
          </tr>
        {/each}
        {#if users.length === 0}
          <tr><td colspan="3" class="px-6 py-12 text-center text-slate-500 italic font-medium leading-relaxed">No users found. Start by adding one.</td></tr>
        {/if}
      </tbody>
    </table>
  </div>
</div>

{#if showCreateModal}
  <div class="fixed inset-0 z-[110] flex items-center justify-center p-4 bg-black/60 backdrop-blur-md">
    <div class="w-full max-w-lg glass p-8 rounded-3xl border border-white/10 shadow-2xl relative">
      <button 
        onclick={() => showCreateModal = false}
        class="absolute top-6 right-6 text-slate-400 hover:text-white transition-colors"
      >
        <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
        </svg>
      </button>

      <h3 class="text-2xl font-black text-white mb-6">Create New User</h3>
      
      <form onsubmit={(e) => { e.preventDefault(); createUser(); }} class="space-y-4">
        <div>
          <label for="new-fullname" class="block text-xs font-bold text-slate-400 uppercase tracking-widest mb-2">Full Name</label>
          <input id="new-fullname" type="text" bind:value={newUser.name} class="w-full bg-white/5 border border-white/10 rounded-2xl px-5 py-3 text-white focus:outline-none focus:border-sky-500" required>
        </div>
        <div>
          <label for="new-email" class="block text-xs font-bold text-slate-400 uppercase tracking-widest mb-2">Email</label>
          <input id="new-email" type="email" bind:value={newUser.email} class="w-full bg-white/5 border border-white/10 rounded-2xl px-5 py-3 text-white focus:outline-none focus:border-sky-500" required>
        </div>
        <div>
          <label for="new-password" class="block text-xs font-bold text-slate-400 uppercase tracking-widest mb-2">Password</label>
          <input id="new-password" type="password" bind:value={newUser.password} class="w-full bg-white/5 border border-white/10 rounded-2xl px-5 py-3 text-white focus:outline-none focus:border-sky-500" required>
        </div>
        <div>
          <label for="new-role" class="block text-xs font-bold text-slate-400 uppercase tracking-widest mb-2">Assign Role</label>
          <select id="new-role" bind:value={newUser.role} class="w-full bg-[#1e293b] border border-white/10 rounded-2xl px-5 py-3 text-white focus:outline-none focus:border-sky-500">
            <option value="OPERATOR">Operator</option>
            <option value="ADMIN">Administrator</option>
            <option value="CHECKER">Admin Checker</option>
          </select>
        </div>

        <button 
          type="submit" 
          disabled={loading}
          class="w-full bg-sky-500 text-white font-bold py-4 rounded-2xl mt-4 hover:bg-sky-400 transition-all shadow-lg shadow-sky-500/20"
        >
          {loading ? 'Processing...' : 'Add User Account'}
        </button>
      </form>
    </div>
  </div>
{/if}
