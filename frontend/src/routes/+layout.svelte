<script lang="ts">
	import './layout.css';
	import favicon from '$lib/assets/favicon.svg';
  import { page } from '$app/state';
  import { onMount } from 'svelte';
  import { goto } from '$app/navigation';

	let { children } = $props();
  
  // State for sidebar toggle
  let sidebarOpen = $state(false);
  let isLight = $state(false);
  let user = $state<{name: string, role: string} | null>(null);
  let masterOpen = $state(false);

  function toggleSidebar() {
    sidebarOpen = !sidebarOpen;
  }

  function toggleTheme() {
    isLight = !isLight;
    if (isLight) {
      document.documentElement.classList.add('light');
      localStorage.setItem('theme', 'light');
    } else {
      document.documentElement.classList.remove('light');
      localStorage.setItem('theme', 'dark');
    }
  }

  function handleLogout() {
    localStorage.removeItem('token');
    localStorage.removeItem('user');
    goto('/login');
  }

  onMount(() => {
    // Auth check
    const token = localStorage.getItem('token');
    const storedUser = localStorage.getItem('user');
    
    if (storedUser) {
      user = JSON.parse(storedUser);
    }

    if (!token && page.url.pathname !== '/login') {
      goto('/login');
    }

    // Theme check
    const storedTheme = localStorage.getItem('theme');
    if (storedTheme === 'light') {
      isLight = true;
      document.documentElement.classList.add('light');
    }
  });
</script>

<svelte:head>
  <link rel="icon" href={favicon} />
  <link href="https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&display=swap" rel="stylesheet">
</svelte:head>

<div class="flex h-screen overflow-hidden bg-[var(--bg-main)] transition-colors duration-300">

  <!-- Overlay for mobile when sidebar is open -->
  {#if sidebarOpen}
    <!-- svelte-ignore a11y_click_events_have_key_events -->
    <!-- svelte-ignore a11y_no_static_element_interactions -->
    <div 
      class="fixed inset-0 bg-black/50 backdrop-blur-sm z-40 md:hidden"
      onclick={toggleSidebar}>
    </div>
  {/if}

  <!-- Glass Sidebar -->
  <aside 
    class={`fixed inset-y-0 left-0 z-50 w-64 glass-dark flex flex-col justify-between border-r border-[#ffffff20] transition-transform duration-300 ease-in-out ${sidebarOpen ? 'translate-x-0' : '-translate-x-full'} md:relative md:translate-x-0`}>
    <div class="p-6">
      <div class="flex items-center justify-between mb-8">
        <h1 class="text-2xl font-black tracking-tighter">
          <span class="text-blue-500">SJS</span><span class="text-amber-400">PAMA</span>
        </h1>
        <!-- Close button on mobile -->
        <button class="md:hidden text-slate-400 hover:text-white" onclick={toggleSidebar} aria-label="Close sidebar">
          <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
          </svg>
        </button>
      </div>
      
      <nav class="space-y-2">
        <a href="/" class="flex items-center space-x-3 px-4 py-3 rounded-xl text-white font-medium transition-all duration-300 {page.url.pathname === '/' ? 'bg-white/10 shadow-[0_0_15px_rgba(14,165,233,0.2)]' : 'text-slate-300 hover:bg-white/10 hover:text-white'}">
          <!-- Icon -->
          <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 {page.url.pathname === '/' ? 'text-sky-400' : ''}" viewBox="0 0 20 20" fill="currentColor">
            <path d="M10.707 2.293a1 1 0 00-1.414 0l-7 7a1 1 0 001.414 1.414L4 10.414V17a1 1 0 001 1h2a1 1 0 001-1v-2a1 1 0 011-1h2a1 1 0 011 1v2a1 1 0 001 1h2a1 1 0 001-1v-6.586l.293.293a1 1 0 001.414-1.414l-7-7z" />
          </svg>
          <span>Dashboard</span>
        </a>
        <a href="/input" class="flex items-center space-x-3 px-4 py-3 rounded-xl font-medium transition-all duration-300 {page.url.pathname === '/input' ? 'bg-white/10 text-white shadow-[0_0_15px_rgba(14,165,233,0.2)]' : 'text-slate-300 hover:bg-white/10 hover:text-white'}">
          <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 {page.url.pathname === '/input' ? 'text-sky-400' : ''}" viewBox="0 0 20 20" fill="currentColor">
            <path d="M17.414 2.586a2 2 0 00-2.828 0L7 10.172V13h2.828l7.586-7.586a2 2 0 000-2.828z" />
            <path fill-rule="evenodd" d="M2 6a2 2 0 012-2h4a1 1 0 010 2H4v10h10v-4a1 1 0 112 0v4a2 2 0 01-2 2H4a2 2 0 01-2-2V6z" clip-rule="evenodd" />
          </svg>
          <span>Daily Input</span>
        </a>
        <a href="/reports" class="flex items-center space-x-3 px-4 py-3 rounded-xl font-medium transition-all duration-300 {page.url.pathname === '/reports' ? 'bg-white/10 text-white shadow-[0_0_15px_rgba(14,165,233,0.2)]' : 'text-slate-300 hover:bg-white/10 hover:text-white'}">
          <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 {page.url.pathname === '/reports' ? 'text-sky-400' : ''}" viewBox="0 0 20 20" fill="currentColor">
            <path fill-rule="evenodd" d="M3 3a1 1 0 000 2v8a2 2 0 002 2h2.586l-1.293 1.293a1 1 0 101.414 1.414L10 15.414l2.293 2.293a1 1 0 001.414-1.414L12.414 15H15a2 2 0 002-2V5a1 1 0 100-2H3zm11.707 4.707a1 1 0 00-1.414-1.414L10 9.586 8.707 8.293a1 1 0 00-1.414 0l-2 2a1 1 0 101.414 1.414L8 10.414l1.293 1.293a1 1 0 001.414 0l4-4z" clip-rule="evenodd" />
          </svg>
          <span>Reports</span>
        </a>
        <!-- Master Data Group -->
        <div class="space-y-1">
          <button 
            onclick={() => masterOpen = !masterOpen}
            class="w-full flex items-center justify-between px-4 py-3 rounded-xl font-medium transition-all duration-300 {['/employees', '/positions', '/licenses', '/units', '/locations'].includes(page.url.pathname) ? 'bg-white/5 text-white' : 'text-slate-300 hover:bg-white/5 hover:text-white'}"
          >
            <div class="flex items-center space-x-3">
              <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 {['/employees', '/positions', '/licenses', '/units', '/locations'].includes(page.url.pathname) ? 'text-sky-400' : ''}" viewBox="0 0 20 20" fill="currentColor">
                <path d="M7 3a1 1 0 000 2h6a1 1 0 100-2H7zM4 7a1 1 0 011-1h10a1 1 0 110 2H5a1 1 0 01-1-1zM2 11a2 2 0 012-2h12a2 2 0 012 2v4a2 2 0 01-2 2H4a2 2 0 01-2-2v-4z" />
              </svg>
              <span>Master Data</span>
            </div>
            <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4 transition-transform duration-300 {masterOpen ? 'rotate-180' : ''}" viewBox="0 0 20 20" fill="currentColor">
              <path fill-rule="evenodd" d="M5.293 7.293a1 1 0 011.414 0L10 10.586l3.293-3.293a1 1 0 111.414 1.414l-4 4a1 1 0 01-1.414 0l-4-4a1 1 0 010-1.414z" clip-rule="evenodd" />
            </svg>
          </button>
          
          {#if masterOpen || ['/employees', '/positions', '/licenses', '/units', '/locations'].includes(page.url.pathname)}
            <div class="pl-4 space-y-1 mt-1 animate-in slide-in-from-top-2 duration-200">
              <a href="/employees" class="flex items-center space-x-3 px-4 py-2 rounded-lg text-sm transition-all {page.url.pathname === '/employees' ? 'text-sky-400 font-bold' : 'text-slate-400 hover:text-white'}">
                <div class="w-1.5 h-1.5 rounded-full {page.url.pathname === '/employees' ? 'bg-sky-400' : 'bg-slate-600'}"></div>
                <span>Karyawan</span>
              </a>
              <a href="/positions" class="flex items-center space-x-3 px-4 py-2 rounded-lg text-sm transition-all {page.url.pathname === '/positions' ? 'text-sky-400 font-bold' : 'text-slate-400 hover:text-white'}">
                <div class="w-1.5 h-1.5 rounded-full {page.url.pathname === '/positions' ? 'bg-sky-400' : 'bg-slate-600'}"></div>
                <span>Jabatan</span>
              </a>
              <a href="/licenses" class="flex items-center space-x-3 px-4 py-2 rounded-lg text-sm transition-all {page.url.pathname === '/licenses' ? 'text-sky-400 font-bold' : 'text-slate-400 hover:text-white'}">
                <div class="w-1.5 h-1.5 rounded-full {page.url.pathname === '/licenses' ? 'bg-sky-400' : 'bg-slate-600'}"></div>
                <span>License</span>
              </a>
              <a href="/units" class="flex items-center space-x-3 px-4 py-2 rounded-lg text-sm transition-all {page.url.pathname === '/units' ? 'text-sky-400 font-bold' : 'text-slate-400 hover:text-white'}">
                <div class="w-1.5 h-1.5 rounded-full {page.url.pathname === '/units' ? 'bg-sky-400' : 'bg-slate-600'}"></div>
                <span>Alat</span>
              </a>
              <a href="/locations" class="flex items-center space-x-3 px-4 py-2 rounded-lg text-sm transition-all {page.url.pathname === '/locations' ? 'text-sky-400 font-bold' : 'text-slate-400 hover:text-white'}">
                <div class="w-1.5 h-1.5 rounded-full {page.url.pathname === '/locations' ? 'bg-sky-400' : 'bg-slate-600'}"></div>
                <span>Lokasi</span>
              </a>
            </div>
          {/if}
        </div>

        <a href="/assignments" class="flex items-center space-x-3 px-4 py-3 rounded-xl font-medium transition-all duration-300 {page.url.pathname === '/assignments' ? 'bg-white/10 text-white shadow-[0_0_15px_rgba(14,165,233,0.2)]' : 'text-slate-300 hover:bg-white/10 hover:text-white'}">
          <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 {page.url.pathname === '/assignments' ? 'text-sky-400' : ''}" viewBox="0 0 20 20" fill="currentColor">
            <path d="M5 4a1 1 0 00-2 0v7.268a2 2 0 000 3.464V16a1 1 0 102 0v-1.268a2 2 0 000-3.464V4zM11 4a1 1 0 10-2 0v1.268a2 2 0 000 3.464V16a1 1 0 102 0V8.732a2 2 0 000-3.464V4zM16 3a1 1 0 011 1v7.268a2 2 0 010 3.464V16a1 1 0 11-2 0v-1.268a2 2 0 010-3.464V4a1 1 0 011-1z" />
          </svg>
          <span>Pairing Operator</span>
        </a>

        <a href="/profile" class="flex items-center space-x-3 px-4 py-3 rounded-xl font-medium transition-all duration-300 {page.url.pathname === '/profile' ? 'bg-white/10 text-white shadow-[0_0_15px_rgba(14,165,233,0.2)]' : 'text-slate-300 hover:bg-white/10 hover:text-white'}">
          <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 {page.url.pathname === '/profile' ? 'text-sky-400' : ''}" viewBox="0 0 20 20" fill="currentColor">
            <path fill-rule="evenodd" d="M10 9a3 3 0 100-6 3 3 0 000 6zm-7 9a7 7 0 1114 0H3z" clip-rule="evenodd" />
          </svg>
          <span>Profile</span>
        </a>
        <a href="/users" class="flex items-center space-x-3 px-4 py-3 rounded-xl font-medium transition-all duration-300 {page.url.pathname === '/users' ? 'bg-white/10 text-white shadow-[0_0_15px_rgba(14,165,233,0.2)]' : 'text-slate-300 hover:bg-white/10 hover:text-white'}">
          <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 {page.url.pathname === '/users' ? 'text-sky-400' : ''}" viewBox="0 0 20 20" fill="currentColor">
            <path d="M9 6a3 3 0 11-6 0 3 3 0 016 0zM17 6a3 3 0 11-6 0 3 3 0 016 0zM12.93 17c.046-.327.07-.66.07-1a6.97 6.97 0 00-1.5-4.33A5 5 0 0119 16v1h-6.07zM6 11a5 5 0 015 5v1H1v-1a5 5 0 015-5z" />
          </svg>
          <span>Users</span>
        </a>
      </nav>

      <!-- Theme Toggle -->
      <div class="mt-8 pt-6 border-t border-[#ffffff10]">
        <button 
          onclick={toggleTheme}
          class="w-full flex items-center justify-between px-4 py-3 rounded-xl bg-white/5 hover:bg-white/10 text-slate-300 hover:text-white transition-all duration-300"
          aria-label="Toggle theme"
        >
          <span class="text-sm font-medium">{isLight ? 'Light Mode' : 'Dark Mode'}</span>
          <div class="w-10 h-6 bg-slate-700 rounded-full relative p-1 transition-colors {isLight ? 'bg-sky-500' : ''}">
            <div class="w-4 h-4 bg-white rounded-full transition-transform duration-300 {isLight ? 'translate-x-4' : 'translate-x-0'}"></div>
          </div>
        </button>
      </div>
    </div>
    <div class="p-6 border-t border-[#ffffff10]">
      <div class="flex items-center justify-between">
        <div class="flex items-center space-x-3">
          <div class="w-10 h-10 rounded-full bg-gradient-to-tr from-sky-400 to-sky-600 flex items-center justify-center font-bold text-white shadow-lg">
            {user?.name?.[0].toUpperCase() || 'U'}
          </div>
          <div>
            <p class="text-sm font-semibold text-white truncate max-w-[100px]">{user?.name || 'User'}</p>
            <p class="text-xs text-sky-400">{user?.role || 'Guest'}</p>
          </div>
        </div>
        
        <button 
          onclick={handleLogout}
          aria-label="Logout"
          class="p-2 text-slate-400 hover:text-rose-400 hover:bg-rose-500/10 rounded-lg transition-all"
        >
          <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 16l4-4m0 0l-4-4m4 4H7m6 4v1a3 3 0 01-3 3H6a3 3 0 01-3-3V7a3 3 0 013-3h4a3 3 0 013 3v1" />
          </svg>
        </button>
      </div>
    </div>
  </aside>

  <!-- Main Content -->
  <main class="flex-1 flex flex-col h-screen overflow-hidden">
    <!-- Header -->
    <header class="glass-dark h-16 flex items-center px-6 justify-between md:hidden border-b border-[#ffffff10] z-30">
      <h1 class="text-xl font-black tracking-tighter">
        <span class="text-blue-500">SJS</span><span class="text-amber-400">PAMA</span>
      </h1>
      <button class="text-white hover:text-sky-400 transition-colors" onclick={toggleSidebar} aria-label="Open sidebar">
        <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h16" />
        </svg>
      </button>
    </header>

    <div class="flex-1 overflow-y-auto p-4 md:p-8 relative">
      <!-- Decorative background elements -->
      <div class="absolute top-[-100px] left-[-100px] w-96 h-96 bg-sky-600 rounded-full mix-blend-multiply filter blur-[128px] opacity-20 -z-10 animate-pulse"></div>
      <div class="absolute bottom-[-100px] right-[-100px] w-96 h-96 bg-indigo-600 rounded-full mix-blend-multiply filter blur-[128px] opacity-20 -z-10"></div>
      
      {@render children()}
    </div>
  </main>
</div>
