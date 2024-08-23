<script>
    import EditorJsEditor from '$lib/components/text-editor/EditorJsEditor.svelte'
    import { onMount } from 'svelte';
    import {goto} from '$app/navigation';
    onMount(() => {
          goto('/text-editor/test');  // Redirect to the default tab for Text Editor
      });
  </script>
  
  <!-- <EditorJsEditor /> -->
  <!-- use this as navigation for open document tabs like in vsc -->

  <h1>Editor settings </h1>

  