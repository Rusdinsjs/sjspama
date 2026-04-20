<script lang="ts">
  import { onMount } from 'svelte';
  import { goto } from '$app/navigation';

  let user = $state<{id: string, name: string, email: string, role: string} | null>(null);
  let name = $state('');
  let email = $state('');
  let password = $state('');
  let confirmPassword = $state('');
  
  let loading = $state(false);
  let message = $state({ text: '', type: '' });
  let showPassword = $state(false);
  let showConfirmPassword = $state(false);

  onMount(() => {
    const storedUser = localStorage.getItem('user');
    if (storedUser) {
      user = JSON.parse(storedUser);
      name = user?.name || '';
      email = user?.email || '';
    } else {
      goto('/login');
    }
  });

  async function handleUpdate(e: SubmitEvent) {
    e.preventDefault();
    if (password && password !== confirmPassword) {
      message = { text: 'Passwords do not match', type: 'error' };
      return;
    }

    loading = true;
    message = { text: '', type: '' };

    try {
      const res = await fetch(`http://localhost:8081/api/users/${user?.id}/profile`, {
        method: 'PUT',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ name, email, password: password || undefined })
      });

      if (res.ok) {
        message = { text: 'Profile updated successfully! Please re-login for changes to take effect.', type: 'success' };
        // Update local storage name if changed
        if (user) {
          const updatedUser = { ...user, name, email };
          localStorage.setItem('user', JSON.stringify(updatedUser));
        }
        password = '';
        confirmPassword = '';
      } else {
        const data = await res.json().catch(() => ({ message: 'Failed to update profile' }));
        message = { text: data.message, type: 'error' };
      }
    } catch (err) {
      message = { text: 'Connection error', type: 'error' };
    } finally {
      loading = false;
    }
  }
</script>

<div class="max-w-2xl mx-auto">
  <div class="mb-8">
    <h1 class="text-3xl font-black tracking-tight text-white mb-2">User Profile</h1>
    <p class="text-slate-400">Manage your account settings and credentials</p>
  </div>

  {#if message.text}
    <div class={`mb-6 p-4 rounded-2xl border ${message.type === 'success' ? 'bg-emerald-500/10 border-emerald-500/30 text-emerald-400' : 'bg-rose-500/10 border-rose-500/30 text-rose-400'} scroll-mt-20`}>
      {message.text}
    </div>
  {/if}

  <div class="glass-dark p-8 rounded-3xl border border-white/10 shadow-xl">
    <form onsubmit={handleUpdate} class="space-y-6">
      <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
        <div class="space-y-2">
          <label for="name" class="block text-xs font-bold text-slate-500 uppercase tracking-widest ml-1">Full Name</label>
          <input 
            id="name"
            type="text" 
            bind:value={name}
            class="w-full bg-white/5 border border-white/10 rounded-2xl px-5 py-3 text-white focus:outline-none focus:ring-2 focus:ring-sky-500/40 transition-all"
            required
          >
        </div>
        <div class="space-y-2">
          <label for="email" class="block text-xs font-bold text-slate-500 uppercase tracking-widest ml-1">Email Address</label>
          <input 
            id="email"
            type="email" 
            bind:value={email}
            class="w-full bg-white/5 border border-white/10 rounded-2xl px-5 py-3 text-white focus:outline-none focus:ring-2 focus:ring-sky-500/40 transition-all"
            required
          >
        </div>
      </div>

      <div class="pt-6 border-t border-white/5 mt-6">
        <h3 class="text-sm font-bold text-sky-400 mb-4 uppercase tracking-wider">Change Password</h3>
        <p class="text-xs text-slate-500 mb-6 italic">Leave blank if you don't want to change your password.</p>
        
        <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
          <div class="space-y-2">
            <label for="password" class="block text-xs font-bold text-slate-500 uppercase tracking-widest ml-1">New Password</label>
            <div class="relative">
              <input 
                id="password"
                type={showPassword ? 'text' : 'password'} 
                bind:value={password}
                placeholder="••••••••"
                class="w-full bg-white/5 border border-white/10 rounded-2xl px-5 py-3 text-white focus:outline-none focus:ring-2 focus:ring-sky-500/40 transition-all pr-12"
              >
              <button 
                type="button"
                onclick={() => showPassword = !showPassword}
                class="absolute right-4 top-1/2 -translate-y-1/2 text-slate-500 hover:text-sky-400 transition-colors"
                aria-label={showPassword ? 'Hide password' : 'Show password'}
              >
                {#if showPassword}
                  <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" viewBox="0 0 20 20" fill="currentColor">
                    <path d="M10 12a2 2 0 100-4 2 2 0 000 4z" />
                    <path fill-rule="evenodd" d="M.458 10C1.732 5.943 5.522 3 10 3s8.268 2.943 9.542 7c-1.274 4.057-5.064 7-9.542 7S1.732 14.057.458 10zM14 10a4 4 0 11-8 0 4 4 0 018 0z" clip-rule="evenodd" />
                  </svg>
                {:else}
                  <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" viewBox="0 0 20 20" fill="currentColor">
                    <path fill-rule="evenodd" d="M3.707 2.293a1 1 0 00-1.414 1.414l14 14a1 1 0 001.414-1.414l-1.473-1.473A10.014 10.014 0 0019.542 10C18.268 5.943 14.478 3 10 3a9.958 9.958 0 00-4.512 1.074l-1.78-1.781zm4.261 4.26l1.514 1.515a2.003 2.003 0 012.45 2.45l1.514 1.514a4 4 0 00-5.478-5.478z" clip-rule="evenodd" />
                    <path d="M12.454 16.697L9.75 13.992a4 4 0 01-3.742-3.741L2.335 6.578A9.98 9.98 0 00.458 10c1.274 4.057 5.065 7 9.542 7 .847 0 1.669-.105 2.454-.303z" />
                  </svg>
                {/if}
              </button>
            </div>
          </div>
          <div class="space-y-2">
            <label for="confirm" class="block text-xs font-bold text-slate-500 uppercase tracking-widest ml-1">Confirm Password</label>
            <div class="relative">
              <input 
                id="confirm"
                type={showConfirmPassword ? 'text' : 'password'} 
                bind:value={confirmPassword}
                placeholder="••••••••"
                class="w-full bg-white/5 border border-white/10 rounded-2xl px-5 py-3 text-white focus:outline-none focus:ring-2 focus:ring-sky-500/40 transition-all pr-12"
              >
              <button 
                type="button"
                onclick={() => showConfirmPassword = !showConfirmPassword}
                class="absolute right-4 top-1/2 -translate-y-1/2 text-slate-500 hover:text-sky-400 transition-colors"
                aria-label={showConfirmPassword ? 'Hide password' : 'Show password'}
              >
                {#if showConfirmPassword}
                  <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" viewBox="0 0 20 20" fill="currentColor">
                    <path d="M10 12a2 2 0 100-4 2 2 0 000 4z" />
                    <path fill-rule="evenodd" d="M.458 10C1.732 5.943 5.522 3 10 3s8.268 2.943 9.542 7c-1.274 4.057-5.064 7-9.542 7S1.732 14.057.458 10zM14 10a4 4 0 11-8 0 4 4 0 018 0z" clip-rule="evenodd" />
                  </svg>
                {:else}
                  <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" viewBox="0 0 20 20" fill="currentColor">
                    <path fill-rule="evenodd" d="M3.707 2.293a1 1 0 00-1.414 1.414l14 14a1 1 0 001.414-1.414l-1.473-1.473A10.014 10.014 0 0019.542 10C18.268 5.943 14.478 3 10 3a9.958 9.958 0 00-4.512 1.074l-1.78-1.781zm4.261 4.26l1.514 1.515a2.003 2.003 0 012.45 2.45l1.514 1.514a4 4 0 00-5.478-5.478z" clip-rule="evenodd" />
                    <path d="M12.454 16.697L9.75 13.992a4 4 0 01-3.742-3.741L2.335 6.578A9.98 9.98 0 00.458 10c1.274 4.057 5.065 7 9.542 7 .847 0 1.669-.105 2.454-.303z" />
                  </svg>
                {/if}
              </button>
            </div>
          </div>
        </div>
      </div>

      <div class="pt-6 flex justify-end">
        <button 
          type="submit" 
          disabled={loading}
          class="bg-sky-600 hover:bg-sky-500 text-white font-bold px-8 py-3 rounded-2xl shadow-lg shadow-sky-500/20 transition-all active:scale-95 disabled:opacity-50"
        >
          {loading ? 'Saving Changes...' : 'Save Profile'}
        </button>
      </div>
    </form>
  </div>
</div>
