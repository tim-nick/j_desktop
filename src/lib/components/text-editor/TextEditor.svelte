<!-- src/lib/components/TextEditor.svelte -->
<script>
    let content = "";
</script>

<div class="editor-container">
    <div
        contenteditable="true"
        bind:innerHTML={content}
        class="editor"
        on:input={(e) => content = e.target.innerHTML}
    ></div>
</div>

<style>
    /* Reset styles for the editor container */
    .editor-container * {
        all: unset;
        box-sizing: border-box;
    }

    /* Apply the editor's own styles */
    .editor-container .editor {
        width: 100%;
        height: 100%;
        background-color: white;
        color: black;
        padding: 10px;
        border-radius: 5px;
        overflow-y: auto;
        /* Apply any other custom styles for your editor */
    }

    /* Optional: Add specific styles for paragraphs or other inline elements */
    .editor-container .editor p {
        margin: 1em 0;
    }

    .editor-container .editor h1 {
        font-size: 2em;
        margin-bottom: 0.5em;
    }

    .editor-container .editor h2 {
        font-size: 1.75em;
        margin-bottom: 0.5em;
    }

    /* Add other custom styles as needed */
</style>