<script>
    import { invoke } from '@tauri-apps/api/tauri';
    export let folder;

    function handleClick(item) {
        invoke('folder_clicked', { name: item.name })
            .then((response) => {
                console.log('Rust function response:', response);
            })
            .catch((error) => {
                console.error('Error invoking Rust function:', error);
            });
    }
</script>

<details>
    <summary on:click={() => handleClick(folder)}>{folder.name}</summary>
    
    {#if folder.subfolders.length > 0}
        <ul>
            {#each folder.subfolders as subfolder}
                <li>
                    <svelte:self folder={subfolder} />
                </li>
            {/each}
        </ul>
    {/if}

    {#if folder.documents.length > 0}
        <ul>
            {#each folder.documents as doc}
                <li><a href="#" on:click={() => handleClick(doc)}>{doc}</a></li>
            {/each}
        </ul>
    {/if}
</details>

<style>
    details > summary {
        cursor: pointer;
        list-style: none;
    }

    ul {
        list-style: none;
        padding-left: 20px;
    }

    a {
        text-decoration: none;
        color: inherit;
        cursor: pointer;
    }
</style>