<script lang="ts">
  import { onMount } from 'svelte';
  import { API } from '../lib/api';
  import { store } from '../lib/store.svelte';
  import { ICONS } from '../lib/constants';
  import type { ConflictEntry } from '../lib/types';
  import Skeleton from '../components/Skeleton.svelte';
  import './WinnowingTab.css';

  // Material Web Imports
  import '@material/web/textfield/outlined-text-field.js';
  import '@material/web/chips/chip-set.js';
  import '@material/web/chips/filter-chip.js';
  import '@material/web/icon/icon.js';

  let conflicts = $state<ConflictEntry[]>([]);
  let loading = $state(true);
  let searchTerm = $state("");

  async function loadData() {
    loading = true;
    try {
      conflicts = await API.getConflicts();
    } catch (e) {
      console.error(e);
      store.showToast("Failed to load conflicts", "error");
    } finally {
      loading = false;
    }
  }

  async function selectWinner(item: ConflictEntry, moduleId: string) {
    // Optimistic Update
    const idx = conflicts.findIndex(c => c.path === item.path);
    if (idx !== -1) {
      conflicts[idx].selected = moduleId;
      conflicts[idx].is_forced = true;
    }
    
    try {
      await API.setWinnowingRule(item.path, moduleId);
    } catch(e) {
      console.error(e);
      store.showToast("Failed to set rule", "error");
    }
  }

  let filteredConflicts = $derived(conflicts.filter(c => 
    c.path.toLowerCase().includes(searchTerm.toLowerCase())
  ));

  onMount(loadData);
</script>

<div class="winnow-container">
  <div class="header">
    <h2>{store.L.tabs.winnowing || 'Winnowing'}</h2>
    <p class="desc">{store.L.common.path || 'Conflict Resolution'}</p>
    
    <div class="search-section">
      <md-outlined-text-field
        label={store.L.modules.searchPlaceholder || "Search paths..."}
        value={searchTerm}
        oninput={(e: Event) => searchTerm = (e.target as HTMLInputElement).value}
        class="full-width-field"
      >
        <md-icon slot="leading-icon"><svg viewBox="0 0 24 24"><path d={ICONS.search} /></svg></md-icon>
      </md-outlined-text-field>
    </div>
  </div>

  {#if loading}
    {#each Array(4) as _}
        <div style="margin-bottom: 0.8rem;">
            <Skeleton height="100px" borderRadius="12px" />
        </div>
    {/each}
  {:else if conflicts.length === 0}
    <div class="clean-state">
      <span class="clean-icon">🌾</span>
      <h3>No Conflicts Detected</h3>
      <p>{store.L.status.healthy || "System is clean."}</p>
    </div>
  {:else}
    <div class="conflict-list">
      {#each filteredConflicts as item}
        <div class="conflict-card" class:forced={item.is_forced}>
          <div class="path-section">
            <div class="path-icon">
              <md-icon><svg viewBox="0 0 24 24"><path d={ICONS.description} /></svg></md-icon>
            </div>
            <div class="path-text" title={item.path}>
              {item.path}
            </div>
          </div>
          
          <md-chip-set class="chip-group">
            {#each item.contenders as modId}
              <md-filter-chip 
                label={modId}
                selected={item.selected === modId}
                onclick={() => selectWinner(item, modId)}
                role="button"
                tabindex="0"
                onkeydown={() => {}}
              ></md-filter-chip>
            {/each}
          </md-chip-set>
        </div>
      {/each}
    </div>
  {/if}
</div>