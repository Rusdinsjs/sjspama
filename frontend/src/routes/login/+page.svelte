<script lang="ts">
  import { goto } from '$app/navigation';

  let email = $state('');
  let password = $state('');
    let errorMsg = $state('');
    let loading = $state(false);
    let showPassword = $state(false);
  
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
        <div class="relative">
          <input 
            id="password"
            type={showPassword ? 'text' : 'password'} 
            bind:value={password}
            placeholder="••••••••"
            class="w-full bg-white/5 border border-white/10 rounded-2xl px-5 py-4 text-white placeholder-slate-600 focus:outline-none focus:ring-2 focus:ring-sky-500/40 focus:border-sky-500 transition-all duration-300 pr-12"
            required
          >
          <button 
            type="button"
            onclick={() => showPassword = !showPassword}
            class="absolute right-4 top-1/2 -translate-y-1/2 text-slate-500 hover:text-sky-400 transition-colors"
            aria-label={showPassword ? 'Hide password' : 'Show password'}
          >
            {#if showPassword}
              <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" viewBox="0 0 20 20" fill="currentColor">
                <path d="M10 12a2 2 0 100-4 2 2 0 000 4z" />
                <path fill-rule="evenodd" d="M.458 10C1.732 5.943 5.522 3 10 3s8.268 2.943 9.542 7c-1.274 4.057-5.064 7-9.542 7S1.732 14.057.458 10zM14 10a4 4 0 11-8 0 4 4 0 018 0z" clip-rule="evenodd" />
              </svg>
            {:else}
              <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" viewBox="0 0 20 20" fill="currentColor">
                <path fill-rule="evenodd" d="M3.707 2.293a1 1 0 00-1.414 1.414l14 14a1 1 0 001.414-1.414l-1.473-1.473A10.014 10.014 0 0019.542 10C18.268 5.943 14.478 3 10 3a9.958 9.958 0 00-4.512 1.074l-1.78-1.781zm4.261 4.26l1.514 1.515a2.003 2.003 0 012.45 2.45l1.514 1.514a4 4 0 00-5.478-5.478z" clip-rule="evenodd" />
                <path d="M12.454 16.697L9.75 13.992a4 4 0 01-3.742-3.741L2.335 6.578A9.98 9.98 0 00.458 10c1.274 4.057 5.065 7 9.542 7 .847 0 1.669-.105 2.454-.303z" />
              </svg>
            {/if}
          </button>
        </div>
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
