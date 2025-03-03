<script lang="ts">
  import { onMount } from 'svelte';
  import { invoke } from '@tauri-apps/api/tauri';
  import FolderItem from '$lib/components/layout/Sidebar/FolderItem.svelte';

  let folders = [];

  async function fetchFolders() {
      try {
          // Call the Tauri command
          const flatFolders = await invoke('fetch_folders_command');
          console.log('Fetched folders:', flatFolders);

          // Convert flat folder list to a nested structure
          return buildFolderTree(flatFolders);
      } catch (error) {
          console.error('Failed to fetch folders:', error);
          return [];
      }
    }


    async function createNewFolder() {
        const folderName = document.getElementById("inputName").value.trim();
        const parentIdRaw = document.getElementById("inputParentId").value.trim();

        // Convert parentId to an integer if valid, otherwise use null.
        const parentId = parentIdRaw ? parseInt(parentIdRaw, 10) : null;

        // Use camelCase key: Tauri will convert it to snake_case in Rust.
        const payload = { name: folderName, parentId: parentId };

        console.log("Payload sent to Rust:", payload);

        try {
            await invoke("create_new_folder_command", payload);
        } catch (error) {
            console.error("Error calling backend:", error);
        }
    }






  function buildFolderTree(flatFolders) {
      const folderMap = new Map();

      // Initialize map with all folders
      flatFolders.forEach(folder => {
          folderMap.set(folder.id, { ...folder, subfolders: [] });
      });

      const rootFolders = [];

      // Organize folders into a tree
      flatFolders.forEach(folder => {
          if (folder.parent_id) {
              const parent = folderMap.get(folder.parent_id);
              if (parent) {
                  parent.subfolders.push(folderMap.get(folder.id));
              }
          } else {
              rootFolders.push(folderMap.get(folder.id));
          }
      });

      return rootFolders;
  }

  onMount(async () => {
      folders = await fetchFolders();
  });
</script>

<main>
    <div>
        <input id="inputName" type="text" placeholder="Folder Name">
        <input id="inputParentId" type="text" placeholder="id of parent folder if folder has parent">
        <button on:click={createNewFolder}>Create Folder</button>
    </div>
  <div>
      {#if folders.length > 0}
          <ul>
              {#each folders as folder}
                  <li><FolderItem {folder} /></li>
              {/each}
          </ul>
      {:else}
          <p>No folders available.</p>
      {/if}
  </div>
</main>

<style>
  ul {
      list-style: none;
      padding-left: 20px;
  }
  #inputName{
    color: black;
  }
  #inputParentId{
    color: black;
  }
</style>