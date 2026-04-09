<script lang="ts">
  import { goto } from '$app/navigation';

  let email = $state('');
  let password = $state('');
    let errorMsg = $state('');
    let loading = $state(false);
  
    const API_BASE = `http://${typeof window !== 'undefined' ? window.location.hostname : '127.0.0.1'}:8081`;
  
    async function handleLogin(e: SubmitEvent) {
    e.preventDefault();
    loading = true;
    errorMsg = '';

    try {
      const res = await fetch(`${API_BASE}/api/auth/login`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ email, password })
      });

      if (res.ok) {
        const data = await res.json();
        localStorage.setItem('token', data.token);
        localStorage.setItem('user', JSON.stringify(data.user));
        goto('/');
      } else {
        const data = await res.json().catch(() => ({ message: 'Invalid email or password' }));
        errorMsg = data.message;
      }
    } catch (e) {
      errorMsg = 'Could not connect to the auth server.';
    } finally {
      loading = false;
    }
  }
</script>

<svelte:head>
  <title>Login - SJS PAMA</title>
</svelte:head>

<div class="fixed inset-0 flex items-center justify-center p-4 bg-[var(--bg-main)] z-[100]">
  <div class="absolute inset-0 overflow-hidden -z-10 opacity-30">
    <div class="absolute top-[10%] right-[15%] w-96 h-96 bg-sky-600/30 rounded-full blur-[120px] animate-pulse"></div>
    <div class="absolute bottom-[20%] left-[10%] w-80 h-80 bg-blue-600/20 rounded-full blur-[100px]"></div>
  </div>

  <div class="w-full max-w-md glass p-10 rounded-3xl shadow-2xl border border-white/10">
    <div class="text-center mb-10">
      <h1 class="text-4xl font-black tracking-tighter mb-2">
        <span class="text-blue-500">SJS</span><span class="text-amber-400">PAMA</span>
      </h1>
      <p class="text-slate-400 font-medium">Digital Operational Excellence</p>
    </div>

    {#if errorMsg}
      <div class="mb-6 p-4 bg-rose-500/10 border border-rose-500/30 text-rose-400 text-sm rounded-2xl font-medium animate-shake">
        {errorMsg}
      </div>
    {/if}

    <form onsubmit={handleLogin} class="space-y-6">
      <div>
        <label for="email" class="block text-xs font-bold text-slate-500 uppercase tracking-widest mb-2 ml-1">Email Address</label>
        <input 
          id="email"
          type="email" 
          bind:value={email}
          placeholder="admin@sjs.id"
          class="w-full bg-white/5 border border-white/10 rounded-2xl px-5 py-4 text-white placeholder-slate-600 focus:outline-none focus:ring-2 focus:ring-sky-500/40 focus:border-sky-500 transition-all duration-300"
          required
        >
      </div>

      <div>
        <label for="password" class="block text-xs font-bold text-slate-500 uppercase tracking-widest mb-2 ml-1">Password</label>
        <input 
          id="password"
          type="password" 
          bind:value={password}
          placeholder="••••••••"
          class="w-full bg-white/5 border border-white/10 rounded-2xl px-5 py-4 text-white placeholder-slate-600 focus:outline-none focus:ring-2 focus:ring-sky-500/40 focus:border-sky-500 transition-all duration-300"
          required
        >
      </div>

      <button 
        type="submit" 
        disabled={loading}
        class="w-full bg-gradient-to-r from-sky-500 to-sky-600 hover:from-sky-400 hover:to-sky-500 text-white font-bold py-4 rounded-2xl transition-all duration-300 shadow-lg shadow-sky-500/20 active:scale-[0.98] disabled:opacity-50"
      >
        {loading ? 'Authenticating...' : 'Sign In'}
      </button>
    </form>
    
    <div class="mt-8 text-center pt-6 border-t border-white/5">
      <p class="text-xs text-slate-500">
        System Access restricted to SJS-PAMA authorized personnel.
      </p>
    </div>
  </div>
</div>

<style>
  @keyframes shake {
    0%, 100% { transform: translateX(0); }
    25% { transform: translateX(-4px); }
    75% { transform: translateX(4px); }
  }
  .animate-shake {
    animation: shake 0.4s ease-in-out;
  }
</style>
