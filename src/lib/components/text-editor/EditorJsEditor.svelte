<script>
  import { onMount } from 'svelte';
  import EditorJS from '@editorjs/editorjs';
  import Header from '@editorjs/header';
  import List from '@editorjs/list';

  let editor;

  onMount(() => {
    editor = new EditorJS({
      holder: 'editorjs',
      tools: {
        header: {
          class: Header,
          inlineToolbar: ['link'],
        },
        list: {
          class: List,
          inlineToolbar: true,
        },
      },
      onReady: () => {
        console.log('Editor.js is ready to work!');
      },
      data: { blocks: [] },
    });
  });
</script>

<div id="editorjs"></div>

<style>
  /* Make sure the styles inside the editor container aren't affected by global TailwindCSS rules */
  :global(#editorjs) {
    width: 100%;
    height: 100%;
    border: 1px solid #ccc;
    border-radius: 5px;
    background-color: #ffffff; /* Set a default background color */
    color: black;
  }

  :global(#editorjs .ce-block) {
    margin-bottom: 20px;
  }

  :global(#editorjs .ce-header) {
    font-size: 2em;
    font-weight: bold;
    margin: 20px 0;
  }

  :global(#editorjs .ce-paragraph) {
    font-size: 1em;
    line-height: 1.5;
  }
</style>