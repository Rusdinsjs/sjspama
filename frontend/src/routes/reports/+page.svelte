<script lang="ts">
  import { onMount } from 'svelte';

  let units = [
    { cn: 'D85ESS-2', pa: 92.5, wh: 210, ipu_eligible: true, invoice: '$12,500' },
    { cn: 'PC200-8', pa: 88.0, wh: 185, ipu_eligible: false, invoice: '$10,200' },
    { cn: 'GD535-5', pa: 95.2, wh: 230, ipu_eligible: true, invoice: '$14,100' },
  ];

  let selectedMonth = '2026-04';
</script>

<svelte:head>
  <title>Reports & Invoicing - SJS PAMA</title>
</svelte:head>

<div class="mb-8 flex justify-between items-end">
  <div>
    <h2 class="text-3xl font-bold text-white tracking-wide">Monthly Reports</h2>
    <p class="text-slate-400 mt-1">Invoice calculations, IPU 1%, and Availability Penalty</p>
  </div>
  <div>
    <input type="month" bind:value={selectedMonth} class="bg-[#ffffff0a] border border-[#ffffff20] rounded-xl px-4 py-2 text-white focus:outline-none focus:border-indigo-500 transition-colors">
  </div>
</div>

<div class="grid grid-cols-1 lg:grid-cols-3 gap-6 mb-8">
  <!-- IPU 1% Summary Card -->
  <div class="glass-dark p-6 rounded-2xl relative overflow-hidden group hover:border-indigo-500/50 transition-all duration-300">
    <div class="absolute top-0 right-0 p-4 opacity-20">
      <svg xmlns="http://www.w3.org/2000/svg" class="h-16 w-16 text-indigo-400" fill="none" viewBox="0 0 24 24" stroke="currentColor">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8c-1.657 0-3 .895-3 2s1.343 2 3 2 3 .895 3 2-1.343 2-3 2m0-8c1.11 0 2.08.402 2.599 1M12 8V7m0 1v8m0 0v1m0-1c-1.11 0-2.08-.402-2.599-1M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
      </svg>
    </div>
    <h3 class="text-lg font-bold text-white mb-2">Total Estimated Invoice</h3>
    <p class="text-4xl font-black text-indigo-400 mb-4">$36,800</p>
    <div class="flex items-center text-sm text-slate-300">
      <span class="bg-indigo-500/20 text-indigo-300 px-2.5 py-0.5 rounded-full mr-2">2 Units</span> eligible for IPU 1%
    </div>
  </div>

  <div class="glass-dark p-6 rounded-2xl relative overflow-hidden group hover:border-rose-500/50 transition-all duration-300">
    <h3 class="text-lg font-bold text-white mb-2">Fuel Consumptions</h3>
    <p class="text-4xl font-black text-rose-400 mb-4">+ 120 L</p>
    <div class="flex justify-between items-center text-sm text-slate-300">
      <span>Excess fuel penalty check</span>
      <button class="text-rose-400 hover:text-white transition">Review Data →</button>
    </div>
  </div>
</div>

<div class="glass rounded-2xl overflow-hidden border border-[#ffffff15] shadow-2xl">
  <div class="p-6 border-b border-[#ffffff10] flex justify-between items-center bg-[#ffffff05]">
    <h3 class="text-xl font-bold text-white">Unit Invoicing & Performance</h3>
    <button class="px-4 py-2 bg-indigo-500/20 text-indigo-300 rounded-lg hover:bg-indigo-500/30 transition-colors text-sm font-medium border border-indigo-500/30">
      Export PDF
    </button>
  </div>
  <div class="overflow-x-auto">
    <table class="w-full text-left text-sm whitespace-nowrap">
      <thead class="text-slate-400 bg-[#ffffff05] border-b border-[#ffffff10]">
        <tr>
          <th class="px-6 py-4 font-semibold">Unit CN</th>
          <th class="px-6 py-4 font-semibold">PA (%)</th>
          <th class="px-6 py-4 font-semibold">Total WH</th>
          <th class="px-6 py-4 font-semibold">IPU 1% Eligible</th>
          <th class="px-6 py-4 font-semibold text-right">Est. Invoice</th>
        </tr>
      </thead>
      <tbody class="divide-y divide-[#ffffff0a]">
        {#each units as unit}
        <tr class="hover:bg-[#ffffff05] transition-colors">
          <td class="px-6 py-4 font-medium text-white">{unit.cn}</td>
          <td class="px-6 py-4 text-slate-300">
            <span class={unit.pa >= 90 ? 'text-emerald-400 font-semibold' : 'text-rose-400 font-semibold'}>{unit.pa}%</span>
          </td>
          <td class="px-6 py-4 text-slate-300">{unit.wh} Hours</td>
          <td class="px-6 py-4">
            {#if unit.ipu_eligible}
              <span class="px-2.5 py-1 rounded-full bg-indigo-400/10 text-indigo-400 border border-indigo-400/20 text-xs font-bold">Yes</span>
            {:else}
              <span class="px-2.5 py-1 rounded-full bg-slate-500/10 text-slate-400 border border-slate-500/20 text-xs font-medium">No</span>
            {/if}
          </td>
          <td class="px-6 py-4 text-right font-medium text-white">
            {unit.invoice}
          </td>
        </tr>
        {/each}
      </tbody>
    </table>
  </div>
</div>
