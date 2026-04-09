<svelte:head>
  <title>Dashboard - SJS PAMA</title>
</svelte:head>

<div class="mb-8">
  <h2 class="text-3xl font-bold text-white tracking-wide">Overview</h2>
  <p class="text-slate-400 mt-1">Daily operations and unit performance summary</p>
</div>

<!-- Metrics Overview -->
<div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6 mb-8">
  <!-- Card 1 -->
  <div class="glass p-6 rounded-2xl relative overflow-hidden group hover:border-sky-500/50 transition-all duration-300">
    <div class="absolute top-0 right-0 p-4 opacity-20 group-hover:opacity-40 transition-opacity">
      <svg xmlns="http://www.w3.org/2000/svg" class="h-16 w-16 text-sky-400" viewBox="0 0 20 20" fill="currentColor">
        <path d="M13 6a3 3 0 11-6 0 3 3 0 016 0zM18 8a2 2 0 11-4 0 2 2 0 014 0zM14 15a4 4 0 00-8 0v3h8v-3zM6 8a2 2 0 11-4 0 2 2 0 014 0zM16 18v-3a5.972 5.972 0 00-.75-2.906A3.005 3.005 0 0119 15v3h-3zM4.75 12.094A5.973 5.973 0 004 15v3H1v-3a3 3 0 013.75-2.906z" />
      </svg>
    </div>
    <p class="text-sm font-medium text-slate-400">Total Active Units</p>
    <p class="text-4xl font-bold text-white mt-2">124</p>
    <div class="mt-4 flex items-center text-sm">
      <span class="text-sky-400 flex items-center bg-sky-400/10 px-2 py-0.5 rounded-full">
        <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4 mr-1" viewBox="0 0 20 20" fill="currentColor">
          <path fill-rule="evenodd" d="M12.586 4.586a2 2 0 112.828 2.828l-3 3a2 2 0 01-2.828 0 1 1 0 00-1.414 1.414 4 4 0 005.656 0l3-3a4 4 0 00-5.656-5.656l-1.5 1.5a1 1 0 101.414 1.414l1.5-1.5zm-5 5a2 2 0 012.828 0 1 1 0 101.414-1.414 4 4 0 00-5.656 0l-3 3a4 4 0 105.656 5.656l1.5-1.5a1 1 0 10-1.414-1.414l-1.5 1.5a2 2 0 11-2.828-2.828l3-3z" clip-rule="evenodd" />
        </svg>
        +3.2%
      </span>
      <span class="text-slate-500 ml-2">from last month</span>
    </div>
  </div>

  <!-- Card 2 -->
  <div class="glass p-6 rounded-2xl relative overflow-hidden group hover:border-blue-500/50 transition-all duration-300">
    <div class="absolute top-0 right-0 p-4 opacity-20 group-hover:opacity-40 transition-opacity">
      <svg xmlns="http://www.w3.org/2000/svg" class="h-16 w-16 text-blue-400" fill="none" viewBox="0 0 24 24" stroke="currentColor">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z" />
      </svg>
    </div>
    <p class="text-sm font-medium text-slate-400">Total Working Hours</p>
    <p class="text-4xl font-bold text-white mt-2">8,240</p>
    <div class="mt-4 flex items-center text-sm hidden">
      <!-- hidden stats -->
    </div>
  </div>

  <!-- Card 3 -->
  <div class="glass p-6 rounded-2xl relative overflow-hidden group hover:border-amber-500/50 transition-all duration-300">
    <div class="absolute top-0 right-0 p-4 opacity-20 group-hover:opacity-40 transition-opacity">
      <svg xmlns="http://www.w3.org/2000/svg" class="h-16 w-16 text-amber-400" viewBox="0 0 20 20" fill="currentColor">
        <path d="M13 6a3 3 0 11-6 0 3 3 0 016 0zM18 8a2 2 0 11-4 0 2 2 0 014 0zM14 15a4 4 0 00-8 0v3h8v-3zM6 8a2 2 0 11-4 0 2 2 0 014 0zM16 18v-3a5.972 5.972 0 00-.75-2.906A3.005 3.005 0 0119 15v3h-3zM4.75 12.094A5.973 5.973 0 004 15v3H1v-3a3 3 0 013.75-2.906z" />
      </svg>
    </div>
    <p class="text-sm font-medium text-slate-400">Average Availability (PA)</p>
    <p class="text-4xl font-bold text-white mt-2">91.5<span class="text-2xl text-slate-300">%</span></p>
    <div class="mt-4 flex items-center text-sm">
      <span class="text-amber-400 font-medium">Eligible for IPU 1%</span>
    </div>
  </div>
  
  <!-- Card 4 -->
  <div class="glass p-6 rounded-2xl relative overflow-hidden group hover:border-rose-500/50 transition-all duration-300">
    <div class="absolute top-0 right-0 p-4 opacity-20 group-hover:opacity-40 transition-opacity">
      <svg xmlns="http://www.w3.org/2000/svg" class="h-16 w-16 text-rose-400" fill="none" viewBox="0 0 24 24" stroke="currentColor">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z" />
      </svg>
    </div>
    <p class="text-sm font-medium text-slate-400">Breakdown Incidents</p>
    <p class="text-4xl font-bold text-rose-400 mt-2">3</p>
    <div class="mt-4 flex items-center text-sm">
       <span class="text-slate-500">Requires attention</span>
    </div>
  </div>
</div>

<!-- Recent Logs Table -->
<div class="glass rounded-2xl overflow-hidden border border-[#ffffff15] shadow-2xl">
  <div class="p-6 border-b border-[#ffffff10] flex justify-between items-center bg-[#ffffff05]">
    <h3 class="text-xl font-bold text-white">Recent Input Logs</h3>
    <a href="/input" class="px-4 py-2 bg-sky-500/20 text-sky-400 rounded-lg hover:bg-sky-500/30 transition-colors text-sm font-medium border border-sky-500/30">
      Add New
    </a>
  </div>
  <div class="overflow-x-auto">
    <table class="w-full text-left text-sm whitespace-nowrap">
      <thead class="text-slate-400 bg-[#ffffff05] border-b border-[#ffffff10]">
        <tr>
          <th class="px-6 py-4 font-semibold">Date</th>
          <th class="px-6 py-4 font-semibold">Unit CN</th>
          <th class="px-6 py-4 font-semibold">Shift</th>
          <th class="px-6 py-4 font-semibold">Total HM</th>
          <th class="px-6 py-4 font-semibold">WH / STB / BD</th>
          <th class="px-6 py-4 font-semibold text-right">Status</th>
        </tr>
      </thead>
      <tbody class="divide-y divide-[#ffffff0a]">
        {#each [1, 2, 3, 4, 5] as i}
        <tr class="hover:bg-[#ffffff05] transition-colors">
          <td class="px-6 py-4 text-slate-300">2026-04-09</td>
          <td class="px-6 py-4"><span class="font-medium text-white">D85ESS-2</span></td>
          <td class="px-6 py-4">
            <span class="px-2 py-1 rounded bg-blue-500/20 text-blue-300 text-xs font-semibold">Shift 1</span>
          </td>
          <td class="px-6 py-4 text-slate-300">12.0</td>
          <td class="px-6 py-4 text-slate-300">
            <span class="text-sky-400">10.0</span> / <span class="text-amber-400">2.0</span> / <span class="text-rose-400">0.0</span>
          </td>
          <td class="px-6 py-4 text-right">
            <span class="px-2.5 py-1 rounded-full bg-sky-400/10 text-sky-400 border border-sky-400/20 text-xs font-medium">
              Verified
            </span>
          </td>
        </tr>
        {/each}
      </tbody>
    </table>
  </div>
</div>
