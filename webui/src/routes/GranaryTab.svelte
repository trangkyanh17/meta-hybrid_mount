<script lang="ts">
  import { onMount } from 'svelte';
  import { fly } from 'svelte/transition';
  import { API } from '../lib/api';
  import { store } from '../lib/store.svelte';
  import { ICONS } from '../lib/constants';
  import type { Silo } from '../lib/types';
  import Skeleton from '../components/Skeleton.svelte';
  import './GranaryTab.css';
  
  // Material Web Imports
  import '@material/web/button/filled-tonal-button.js';
  import '@material/web/button/text-button.js';
  import '@material/web/iconbutton/icon-button.js';
  import '@material/web/icon/icon.js';
  import '@material/web/dialog/dialog.js';
  import '@material/web/ripple/ripple.js';

  let silos = $state<Silo[]>([]);
  let loading = $state(true);
  let restoringId = $state<string | null>(null);
  
  // Dialog State
  let showConfirmDialog = $state(false);
  let selectedSilo = $state<Silo | null>(null);

  // Missing icon definition
  const ICON_HISTORY = "M13,3A9,9 0 0,0 4,12H1L4.89,15.89L4.96,16.03L9,12H6A7,7 0 0,1 13,5A7,7 0 0,1 20,12A7,7 0 0,1 13,19C11.07,19 9.32,18.2 8.06,16.94L6.64,18.36C8.27,20 10.5,21 13,21A9,9 0 0,0 22,12A9,9 0 0,0 13,3Z";

  async function loadSilos() {
    loading = true;
    try {
      silos = await API.getGranaryList();
    } catch (e) {
      console.error(e);
      store.showToast("Failed to load Granary", "error");
    } finally {
      loading = false;
    }
  }

  function requestRestore(silo: Silo) {
    selectedSilo = silo;
    showConfirmDialog = true;
  }

  async function performRestore() {
    if (!selectedSilo) return;
    showConfirmDialog = false;
    restoringId = selectedSilo.id;
    
    try {
      await API.restoreSilo(selectedSilo.id);
      store.showToast(store.L.common.saveSuccess || "Restored successfully. Please reboot.", "success");
    } catch (e: any) {
      store.showToast("Restore failed: " + e, "error");
    } finally {
      restoringId = null;
      selectedSilo = null;
    }
  }

  function formatTime(ts: number) {
    return new Date(ts * 1000).toLocaleString();
  }

  onMount(loadSilos);
</script>

<md-dialog 
  open={showConfirmDialog} 
  onclose={() => showConfirmDialog = false}
  style="--md-dialog-scrim-color: transparent; --md-sys-color-scrim: transparent;"
>
  <div slot="headline">Restore Snapshot?</div>
  <div slot="content">
    <p>Are you sure you want to restore <strong>{selectedSilo?.label}</strong>?</p>
    <p style="font-size: 0.9rem; opacity: 0.8;">Current configuration will be overwritten.</p>
  </div>
  <div slot="actions">
    <md-text-button 
      onclick={() => showConfirmDialog = false}
      role="button"
      tabindex="0"
      onkeydown={() => {}}
    >
      {store.L.common.cancel || 'Cancel'}
    </md-text-button>
    <md-text-button 
      onclick={performRestore}
      role="button"
      tabindex="0"
      onkeydown={() => {}}
    >
      {store.L.common.confirm || 'Restore'}
    </md-text-button>
  </div>
</md-dialog>

<div class="granary-container">
  <div class="header">
    <h2>{store.L.tabs.granary || 'Granary'} <span class="subtitle">Snapshots</span></h2>
    <md-icon-button 
      onclick={loadSilos} 
      disabled={loading}
      role="button"
      tabindex="0"
      onkeydown={() => {}}
    >
      <md-icon><svg viewBox="0 0 24 24"><path d={ICONS.refresh} /></svg></md-icon>
    </md-icon-button>
  </div>

  {#if loading}
    <div class="list">
      {#each Array(3) as _}
        <div style="margin-bottom: 1rem;">
           <Skeleton height="80px" />
        </div>
      {/each}
    </div>
  {:else if silos.length === 0}
    <div class="empty-state">
      <div class="empty-icon">🌾</div>
      <h3>The Granary is empty</h3>
      <p>Snapshots are created automatically before critical operations.</p>
    </div>
  {:else}
    <div class="list">
      {#each silos as silo (silo.id)}
        <div class="silo-card" transition:fly={{ y: 20, duration: 300 }}>
          <md-ripple></md-ripple>
          <div class="silo-info">
            <div class="silo-label">{silo.label}</div>
            <div class="silo-meta">
              <span class="reason-badge">{silo.reason}</span>
              <span class="time">{formatTime(silo.timestamp)}</span>
            </div>
            <div class="silo-id">{silo.id}</div>
          </div>
          <div class="silo-actions">
            <md-filled-tonal-button 
              onclick={() => requestRestore(silo)}
              disabled={restoringId !== null}
              role="button"
              tabindex="0"
              onkeydown={() => {}}
            >
              {#if restoringId === silo.id}
                 Busy...
              {:else}
                 {store.L.common.confirm || 'Restore'}
              {/if}
              <md-icon slot="icon"><svg viewBox="0 0 24 24"><path d={ICON_HISTORY} /></svg></md-icon>
            </md-filled-tonal-button>
          </div>
        </div>
      {/each}
    </div>
  {/if}
</div>