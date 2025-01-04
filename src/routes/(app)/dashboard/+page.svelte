<script>
    import { onMount } from 'svelte';
    import { invoke } from '@tauri-apps/api/tauri';
  
    let folders = [];

    async function fetchFolders() {
        try {
            // Call the Tauri command
            const folders = await invoke('fetch_folders_command');
            console.log('Fetched folders:', folders);
            return folders;
        } catch (error) {
            console.error('Failed to fetch folders:', error);
            throw error;
        }
    }
  
    onMount(async () => {
      try {
        folders = await fetchFolders();
      } catch (error) {
        console.error('Error loading folders:', error);
      }
    });
  </script>

<main>
    <div>
        {#if folders.length > 0}
          <ul>
            {#each folders as folder}
              <li>{folder.name}</li> <!-- Adjust based on the Folder structure -->
            {/each}
          </ul>
        {:else}
          <p>No folders available.</p>
        {/if}
      </div>
</main>

<style>
    /* Add any necessary CSS here */
</style>