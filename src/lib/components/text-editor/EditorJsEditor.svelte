<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import { invoke } from '@tauri-apps/api/tauri';
  import { get } from 'svelte/store';
  import type { DocumentEditor } from '$lib/stores';
  import {documentsEditorStore } from '$lib/stores';
  import { page } from '$app/stores';

  // Editor JS Tools
  import EditorJS from '@editorjs/editorjs';
  import Header from '@editorjs/header';
  import Checklist from '@editorjs/checklist'; //TODO: implement md parser function for - [] and - [x]
  // import List from '@editorjs/list';
  // import NestedList from '@editorjs/nested-list';
  import NestedList from '$lib/components/text-editor/editor-tools/nested-list'
  import Paragraph from '$lib/components/text-editor/editor-tools/paragraph/src/index'

  import Flashcard from '$lib/components/text-editor/editor-tools/Flashcards';
  // import HeadingTool from '$lib/components/text-editor/editor-tools/HeadingTool'

  //export let id: number;
  export let current_docId: number = 1; // The document ID to be loaded

  let doc: DocumentEditor | undefined;

  // JavaScript variables
  let editor: any; // EditorJS instance EditorJS also isntead of any.

  let url;
  let params;
  let query;
  let queryParams;

  // Subscribe to the page store, query
  $: ({ url, params } = $page);
  $: queryParams = new URLSearchParams(url.search);
  // Initialize Editor.js
  onMount(() => {
    console.log('Current URL:', url);
    console.log('Route Parameters:', params);
    editor = new EditorJS({
      holder: 'editorjs',
      defaultBlock: 'Paragraph',
      tools: {
        header: {
          class: Header as any  , // TypeScript workaround
          shortcut: 'CMD+SHIFT+H',
          inlineToolbar: ['link'],
          config: {
            placeholder: 'Enter a header',
            levels: [1, 2, 3, 4, 5],
            defaultLevel: 1
          }
        },
        Paragraph: {
          class: Paragraph,
          inlineToolbar: true,
          config: {
            placeholder: 'Enter a paragraph',
          }
        },
        // heading: HeadingTool,
        nestedList: {
          class: NestedList as any, // TypeScript workaround
          inlineToolbar: true,
        },
        checklist: {
          class: Checklist,
          inlineToolbar: true,
        },
        flashcard: Flashcard,
      },
      onReady: () => {
        console.log('Editor.js is ready to work!');
        if (current_docId) {
          loadDocument(current_docId); // Load the document when the editor is ready
        }
      },
      data: { blocks: [] },
    });
});


  // First editor db interface 


  function loadDocument(docId: number) {
    const documents = get(documentsEditorStore);
    const document = documents.find(doc => doc.id === docId);

    if (document) {
      console.log("Loading document in Editor:", document);
      // TODO: deserialzie or serialize the document content wright from the database so its json and not  as triong 
      // Ensure that the content is a valid JSON object
      let content;
      try {
        content = typeof document.content === 'string'
          ? JSON.parse(document.content)
          : document.content;

        // Validate content format
        if (content && content.blocks) {
          editor.clear();
          editor.render(content);
        } else {
          console.error("Invalid content format:", content);
        }
      } catch (error) {
        console.error("Error parsing document content:", error);
      }
    } else {
      console.error("Document not found with id:", docId);
    }
  }

  // Save document
  async function saveDocument() {
    try {
      const doc = await editor.save();
      console.log('Saving document:', doc);
      await invoke('save_document_command', { doc });
      console.log('Document saved successfully');
    } catch (error) {
      console.error('Error saving document:', error);
    }
  }


  // Update document
  async function updateDocument() {
    try {
      const doc = await editor.save();
      console.log('Updating document:', doc);
      if (current_docId) {
        await invoke('update_document_command', { id: parseInt(current_docId.toString(), 10), doc: doc });
        console.log('Document updated successfully');
      } else {
        console.log('No ID entered');
      }
    } catch (error) {
      console.error('Error updating document:', error);
    }
  }

  async function newFile() {
    // saveFile();
    editor.clear();
  }

  async function loadFile() {
        console.log("loadFile function called");
        const inputElement = document.getElementById('myInput') as HTMLInputElement;
        // console.log('Prompt returned:', id);
        if (inputElement) {
          const id = inputElement.value;
          if (id) {
            console.log(`Attempting to load document with ID: ${id}`);

            // First look in documentsStore

            try {
              console.log('Fetching documents from store');
              const documents = get(documentsEditorStore);
              doc = documents.find((d) => d.id === parseInt(id, 10));
              console.log('Document found:', doc);
              if (doc) {
                console.log('Document found in store:', doc);
                editor.clear();
                editor.render(doc.content);
                current_docId = doc.id;
                return;
              }
            } catch (error) {
              console.error('Error fetching documents:', error);
            }

            // If not found, try fetching from the backend
            const result = await invoke('load_document_command', { id: parseInt(id, 10) });
            console.log('Loaded document:', result);
            editor.clear();
            // const editorData = JSON.parse(result);
            editor.render(result);
            doc = result as DocumentEditor;
            current_docId = doc.id;
          } else {
            console.log('No ID entered');
          }
        } else {
          console.log('Input element not found');
        }
    };


$: if (current_docId) {
    console.log('Current docId changed:', current_docId);
    loadDocument(current_docId);
  }

</script>

<!-- <p>Current URL: {$page.url.pathname}</p>
<p>Route Parameters: {JSON.stringify($page.params)}</p>
<p>Query Parameters: {JSON.stringify(queryParams)}</p> -->

<button on:click={loadFile}>Open File</button>
<button on:click={saveDocument}>Save File</button>
<button on:click={updateDocument}>Update File</button>
<button on:click={newFile}>New File</button>
<!-- <button on:click={savePythonDoc}>New Python File</button> -->
<input type="text" id="myInput" placeholder="doc-id">
<input type="text" id="folderId" placeholder="folder-id">


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
    /* margin-bottom: 20px; */
    margin-bottom: 0 !important; /* Or set it to a smaller value as necessary */
  }
  :global(#editorjs .ce-block h1) {
  font-size: 2em;
  font-weight: bold;
  padding: 0;
  /* margin: 20px 0; */
  }
  :global(#editorjs h2) {
    font-size: 1.75em;
    font-weight: bold;
    margin: 18px 0;
    padding: 0;
  }
  :global(#editorjs h3) {
    font-size: 1.5em;
    font-weight: bold;
    margin: 16px 0;
    padding: 0;
  }
  :global(#editorjs h4) {
    font-size: 1.25em;
    font-weight: bold;
    margin: 14px 0;
    padding: 0;
  }
  :global(#editorjs h5) {
    font-size: 1.0em;
    font-weight: bold;
    margin: 12px 0;
    padding: 0;
  }
  :global(#editorjs h6) {
    font-size: 0.75em;
    font-weight: bold;
    margin: 10px 0;
    padding: 0;
  }

  :global(#editorjs .ce-paragraph) {
    /* font-size: 1em; */
    /* line-height: 1.5; */
    padding: 0;
    /* text-align: center; */
  }
</style>